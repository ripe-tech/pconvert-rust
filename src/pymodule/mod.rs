mod conversions;
mod utils;

use crate::blending::params::{BlendAlgorithmParams, Options};
use crate::blending::{
    blend_images, demultiply_image, get_blending_algorithm, is_algorithm_multiplied, BlendAlgorithm,
};
use crate::constants;
use crate::errors::PConvertError;
use crate::parallelism::{ResultMessage, ThreadPool};
use crate::utils::{read_png_from_file, write_png_parallel, write_png_to_file};
use pyo3::prelude::*;
use pyo3::types::PySequence;
use std::sync::mpsc;
use utils::{
    build_algorithm, build_params, get_compression_type, get_filter_type, get_num_threads,
};

static mut THREAD_POOL: Option<ThreadPool> = None;

#[pymodule]
fn pconvert_rust(_py: Python, m: &PyModule) -> PyResult<()> {
    unsafe {
        THREAD_POOL = Some(ThreadPool::new(5).unwrap());
    }

    m.add("COMPILATION_DATE", constants::COMPILATION_DATE)?;

    m.add("COMPILATION_TIME", constants::COMPILATION_TIME)?;

    m.add("VERSION", constants::VERSION)?;

    m.add("ALGORITHMS", constants::ALGORITHMS.to_vec())?;

    m.add("COMPILER", constants::COMPILER)?;

    m.add("COMPILER_VERSION", constants::COMPILER_VERSION)?;

    m.add("LIBPNG_VERSION", constants::LIBPNG_VERSION)?;

    m.add("FEATURES", constants::FEATURES.to_vec())?;

    m.add("PLATFORM_CPU_BITS", constants::PLATFORM_CPU_BITS)?;

    let filters: Vec<String> = constants::FILTER_TYPES
        .to_vec()
        .iter()
        .map(|x| format!("{:?}", x))
        .collect();
    m.add("FILTER_TYPES", filters)?;

    let compressions: Vec<String> = constants::COMPRESSION_TYPES
        .to_vec()
        .iter()
        .map(|x| format!("{:?}", x))
        .collect();
    m.add("COMPRESSION_TYPES", compressions)?;

    #[pyfn(m, "blend_images")]
    fn blend_images_py(
        bot_path: String,
        top_path: String,
        target_path: String,
        algorithm: Option<String>,
        is_inline: Option<bool>,
        options: Option<Options>,
    ) -> PyResult<()> {
        let num_threads = get_num_threads(&options);
        if num_threads <= 0 {
            blend_images_single_thread(
                bot_path,
                top_path,
                target_path,
                algorithm,
                is_inline,
                options,
            )
        } else {
            unsafe {
                blend_images_multi_thread(
                    bot_path,
                    top_path,
                    target_path,
                    algorithm,
                    is_inline,
                    options,
                    num_threads,
                )
            }
        }
    }

    #[pyfn(m, "blend_multiple")]
    fn blend_multiple_py(
        img_paths: &PySequence,
        out_path: String,
        algorithm: Option<String>,
        algorithms: Option<&PySequence>,
        is_inline: Option<bool>,
        options: Option<Options>,
    ) -> PyResult<()> {
        let num_threads = get_num_threads(&options);
        if num_threads <= 0 {
            blend_multiple_single_thread(
                img_paths, out_path, algorithm, algorithms, is_inline, options,
            )
        } else {
            blend_multiple_multi_thread(
                img_paths,
                out_path,
                algorithm,
                algorithms,
                is_inline,
                options,
                num_threads,
            )
        }
    }

    Ok(())
}

fn blend_images_single_thread(
    bot_path: String,
    top_path: String,
    target_path: String,
    algorithm: Option<String>,
    is_inline: Option<bool>,
    options: Option<Options>,
) -> PyResult<()> {
    let algorithm = algorithm.unwrap_or(String::from("multiplicative"));
    let algorithm = build_algorithm(&algorithm)?;

    let _is_inline = is_inline.unwrap_or(false);

    let demultiply = is_algorithm_multiplied(&algorithm);
    let algorithm_fn = get_blending_algorithm(&algorithm);

    let mut bot = read_png_from_file(bot_path, demultiply)?;
    let top = read_png_from_file(top_path, demultiply)?;

    blend_images(&top, &mut bot, &algorithm_fn, &None);

    let compression_type = get_compression_type(&options);
    let filter_type = get_filter_type(&options);
    write_png_to_file(target_path, &bot, compression_type, filter_type)?;

    Ok(())
}

unsafe fn blend_images_multi_thread(
    bot_path: String,
    top_path: String,
    target_path: String,
    algorithm: Option<String>,
    is_inline: Option<bool>,
    options: Option<Options>,
    num_threads: i32,
) -> PyResult<()> {
    let algorithm = algorithm.unwrap_or(String::from("multiplicative"));
    let algorithm = build_algorithm(&algorithm)?;
    let _is_inline = is_inline.unwrap_or(false);
    let demultiply = is_algorithm_multiplied(&algorithm);
    let algorithm_fn = get_blending_algorithm(&algorithm);

    let thread_pool = match &mut THREAD_POOL {
        Some(thread_pool) => thread_pool,
        None => panic!("Unable to access global pconvert thread pool")
    };
    thread_pool.start();

    for i in 0..10 {
        thread_pool.execute(move || {
            println!("Hello from thread {}", i);
            std::thread::sleep(std::time::Duration::from_secs(3));
            ResultMessage::ImageResult(Err(PConvertError::UnsupportedImageTypeError))
        });
    }

    let top_result_channel = thread_pool
        .execute(move || ResultMessage::ImageResult(read_png_from_file(top_path, demultiply)));
    let bot_result_channel = thread_pool
        .execute(move || ResultMessage::ImageResult(read_png_from_file(bot_path, demultiply)));

    let top = match top_result_channel.recv().unwrap() {
        ResultMessage::ImageResult(result) => result,
    }?;

    let mut bot = match bot_result_channel.recv().unwrap() {
        ResultMessage::ImageResult(result) => result,
    }?;

    blend_images(&top, &mut bot, &algorithm_fn, &None);

    let compression_type = get_compression_type(&options);
    let filter_type = get_filter_type(&options);
    write_png_parallel(target_path, &bot, compression_type, filter_type)?;

    Ok(())
}

fn blend_multiple_single_thread(
    img_paths: &PySequence,
    out_path: String,
    algorithm: Option<String>,
    algorithms: Option<&PySequence>,
    is_inline: Option<bool>,
    options: Option<Options>,
) -> PyResult<()> {
    let num_images = img_paths.len()? as usize;

    if num_images < 1 {
        return Err(PyErr::from(PConvertError::ArgumentError(
            "ArgumentError: 'img_paths' must contain at least one path".to_string(),
        )));
    }

    if algorithms.is_some() && algorithms.unwrap().len()? != num_images as isize - 1 {
        return Err(PyErr::from(PConvertError::ArgumentError(format!(
            "ArgumentError: 'algorithms' must be of size {} (one per blending operation)",
            num_images - 1
        ))));
    };

    let _is_inline = is_inline.unwrap_or(false);

    let algorithms_to_apply: Vec<(BlendAlgorithm, Option<BlendAlgorithmParams>)> =
        if let Some(algorithms) = algorithms {
            build_params(algorithms)?
        } else if let Some(algorithm) = algorithm {
            let algorithm = build_algorithm(&algorithm)?;
            vec![(algorithm, None); num_images - 1]
        } else {
            vec![(BlendAlgorithm::Multiplicative, None); num_images - 1]
        };

    let mut img_paths_iter = img_paths.iter()?;
    let first_path = img_paths_iter.next().unwrap()?.to_string();
    let first_demultiply = is_algorithm_multiplied(&algorithms_to_apply[0].0);
    let mut composition = read_png_from_file(first_path, first_demultiply)?;
    let mut zip_iter = img_paths_iter.zip(algorithms_to_apply.iter());
    while let Some(pair) = zip_iter.next() {
        let path = pair.0?.extract::<String>()?;
        let (algorithm, algorithm_params) = pair.1;
        let demultiply = is_algorithm_multiplied(&algorithm);
        let algorithm_fn = get_blending_algorithm(&algorithm);
        let current_layer = read_png_from_file(path, demultiply)?;
        blend_images(
            &current_layer,
            &mut composition,
            &algorithm_fn,
            algorithm_params,
        );
    }

    let compression_type = get_compression_type(&options);
    let filter_type = get_filter_type(&options);
    write_png_to_file(out_path, &composition, compression_type, filter_type)?;

    Ok(())
}

fn blend_multiple_multi_thread(
    img_paths: &PySequence,
    out_path: String,
    algorithm: Option<String>,
    algorithms: Option<&PySequence>,
    is_inline: Option<bool>,
    options: Option<Options>,
    num_threads: i32,
) -> PyResult<()> {
    let num_images = img_paths.len()? as usize;

    if num_images < 1 {
        return Err(PyErr::from(PConvertError::ArgumentError(
            "ArgumentError: 'img_paths' must contain at least one path".to_string(),
        )));
    }

    if algorithms.is_some() && algorithms.unwrap().len()? != num_images as isize - 1 {
        return Err(PyErr::from(PConvertError::ArgumentError(format!(
            "ArgumentError: 'algorithms' must be of size {} (one per blending operation)",
            num_images - 1
        ))));
    };

    let _is_inline = is_inline.unwrap_or(false);

    let algorithms_to_apply: Vec<(BlendAlgorithm, Option<BlendAlgorithmParams>)> =
        if let Some(algorithms) = algorithms {
            build_params(algorithms)?
        } else if let Some(algorithm) = algorithm {
            let algorithm = build_algorithm(&algorithm)?;
            vec![(algorithm, None); num_images - 1]
        } else {
            vec![(BlendAlgorithm::Multiplicative, None); num_images - 1]
        };

    let mut thread_pool = ThreadPool::new(num_threads as usize)?;
    thread_pool.start();

    let mut png_channels: Vec<mpsc::Receiver<ResultMessage>> = Vec::with_capacity(num_images);
    for path in img_paths.iter()? {
        let path = path?.to_string();
        let result_channel = thread_pool.execute(move || -> ResultMessage {
            ResultMessage::ImageResult(read_png_from_file(path, false))
        });
        png_channels.push(result_channel);
    }

    let first_demultiply = is_algorithm_multiplied(&algorithms_to_apply[0].0);
    let mut composition = match png_channels[0].recv().unwrap() {
        ResultMessage::ImageResult(result) => result,
    }?;
    if first_demultiply {
        demultiply_image(&mut composition)
    }

    for i in 1..png_channels.len() {
        let (algorithm, algorithm_params) = &algorithms_to_apply[i - 1];
        let demultiply = is_algorithm_multiplied(&algorithm);
        let algorithm_fn = get_blending_algorithm(&algorithm);
        let mut current_layer = match png_channels[i].recv().unwrap() {
            ResultMessage::ImageResult(result) => result,
        }?;
        if demultiply {
            demultiply_image(&mut current_layer)
        }

        blend_images(
            &current_layer,
            &mut composition,
            &algorithm_fn,
            algorithm_params,
        );
    }

    let compression_type = get_compression_type(&options);
    let filter_type = get_filter_type(&options);
    write_png_parallel(out_path, &composition, compression_type, filter_type)?;

    Ok(())
}
