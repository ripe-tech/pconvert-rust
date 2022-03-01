//! Python extension, exported functions and type conversions.

pub mod conversions;
pub mod utils;

use crate::blending::params::{BlendAlgorithmParams, Options};
use crate::blending::{
    blend_images, demultiply_image, get_blending_algorithm, is_algorithm_multiplied, BlendAlgorithm,
};
use crate::constants;
use crate::errors::PConvertError;
use crate::parallelism::{ResultMessage, ThreadPool};
use crate::utils::{read_png_from_file, write_png_parallel, write_png_to_file};
use pyo3::exceptions::PyException;
use pyo3::prelude::*;
use pyo3::types::{IntoPyDict, PyDict, PySequence};
use std::sync::mpsc;
use utils::{
    build_algorithm, build_params, get_compression_type, get_filter_type, get_num_threads,
};

static mut THREAD_POOL: Option<ThreadPool> = None;

#[pymodule]
fn pconvert_rust(_py: Python, module: &PyModule) -> PyResult<()> {
    unsafe {
        let mut thread_pool = ThreadPool::new(constants::DEFAULT_THREAD_POOL_SIZE).unwrap();
        thread_pool.start();
        THREAD_POOL = Some(thread_pool);
    }

    module.add("COMPILATION_DATE", constants::COMPILATION_DATE)?;
    module.add("COMPILATION_TIME", constants::COMPILATION_TIME)?;
    module.add("VERSION", constants::VERSION)?;
    module.add("ALGORITHMS", constants::ALGORITHMS.to_vec())?;
    module.add("COMPILER", constants::COMPILER)?;
    module.add("COMPILER_VERSION", constants::COMPILER_VERSION)?;
    module.add("LIBPNG_VERSION", constants::LIBPNG_VERSION)?;
    module.add("FEATURES", constants::FEATURES.to_vec())?;
    module.add("PLATFORM_CPU_BITS", constants::PLATFORM_CPU_BITS)?;

    let filters: Vec<String> = constants::FILTER_TYPES
        .to_vec()
        .iter()
        .map(|x| format!("{:?}", x))
        .collect();
    module.add("FILTER_TYPES", filters)?;

    let compressions: Vec<String> = constants::COMPRESSION_TYPES
        .to_vec()
        .iter()
        .map(|x| format!("{:?}", x))
        .collect();
    module.add("COMPRESSION_TYPES", compressions)?;

    #[pyfunction]
    #[pyo3(name = "blend_images")]
    fn blend_images_py(
        py: Python,
        bot_path: String,
        top_path: String,
        target_path: String,
        algorithm: Option<String>,
        is_inline: Option<bool>,
        options: Option<Options>,
    ) -> PyResult<()> {
        // blends two images using either the single-threaded or the multiple-threaded version
        // taking into consideration the requested number of thread in options
        py.allow_threads(|| -> PyResult<()> {
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
        })
    }

    #[pyfunction]
    #[pyo3(name = "blend_multiple")]
    fn blend_multiple_py(
        py: Python,
        img_paths: &PySequence,
        out_path: String,
        algorithm: Option<String>,
        algorithms: Option<&PySequence>,
        is_inline: Option<bool>,
        options: Option<Options>,
    ) -> PyResult<()> {
        // parses python types to rust owned values so that they are safely shared between threads
        let img_paths: Vec<String> = img_paths.extract()?;
        let num_images = img_paths.len();

        let algorithms_to_apply: Vec<(BlendAlgorithm, Option<BlendAlgorithmParams>)> =
            if let Some(algorithms) = algorithms {
                build_params(algorithms)?
            } else if let Some(algorithm) = algorithm {
                let algorithm = build_algorithm(&algorithm)?;
                vec![(algorithm, None); num_images - 1]
            } else {
                vec![(BlendAlgorithm::Multiplicative, None); num_images - 1]
            };

        // blends multiple images using either the single-threaded or the multiple-threaded version
        // taking into consideration the requested number of thread in options
        py.allow_threads(|| -> PyResult<()> {
            let num_threads = get_num_threads(&options);
            if num_threads <= 0 {
                blend_multiple_single_thread(
                    img_paths,
                    out_path,
                    algorithms_to_apply,
                    is_inline,
                    options,
                )
            } else {
                unsafe {
                    blend_multiple_multi_thread(
                        img_paths,
                        out_path,
                        algorithms_to_apply,
                        is_inline,
                        options,
                        num_threads,
                    )
                }
            }
        })
    }

    #[pyfunction]
    #[pyo3(name = "get_thread_pool_status")]
    fn get_thread_pool_status(py: Python) -> PyResult<&PyDict> {
        unsafe {
            match &mut THREAD_POOL {
                Some(thread_pool) => {
                    let status_dict = thread_pool.get_status().into_py_dict(py);
                    Ok(status_dict)
                }
                None => Err(PyException::new_err(
                    "Acessing global thread pool".to_string(),
                )),
            }
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
    let algorithm = algorithm.unwrap_or_else(|| String::from("multiplicative"));
    let algorithm = build_algorithm(&algorithm)?;

    let _is_inline = is_inline.unwrap_or(false);

    let demultiply = is_algorithm_multiplied(&algorithm);
    let algorithm_fn = get_blending_algorithm(&algorithm);

    let mut bot = read_png_from_file(bot_path, demultiply)?;
    let top = read_png_from_file(top_path, demultiply)?;

    blend_images(&mut bot, &top, &algorithm_fn, &None);

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
    let algorithm = algorithm.unwrap_or_else(|| String::from("multiplicative"));
    let algorithm = build_algorithm(&algorithm)?;
    let _is_inline = is_inline.unwrap_or(false);
    let demultiply = is_algorithm_multiplied(&algorithm);
    let algorithm_fn = get_blending_algorithm(&algorithm);

    let thread_pool = match &mut THREAD_POOL {
        Some(thread_pool) => thread_pool,
        None => panic!("Unable to access global pconvert thread pool"),
    };

    // expands thread pool to the desired number of threads/parallelism (if necessary and possible)
    thread_pool.expand_to(num_threads as usize);

    let bot_result_channel = thread_pool
        .execute(move || ResultMessage::ImageResult(read_png_from_file(bot_path, demultiply)));
    let top_result_channel = thread_pool
        .execute(move || ResultMessage::ImageResult(read_png_from_file(top_path, demultiply)));

    let mut bot = match bot_result_channel.recv().unwrap() {
        ResultMessage::ImageResult(result) => result,
    }?;
    let top = match top_result_channel.recv().unwrap() {
        ResultMessage::ImageResult(result) => result,
    }?;

    blend_images(&mut bot, &top, &algorithm_fn, &None);

    let compression_type = get_compression_type(&options);
    let filter_type = get_filter_type(&options);
    write_png_parallel(target_path, &bot, compression_type, filter_type)?;

    Ok(())
}

fn blend_multiple_single_thread(
    img_paths: Vec<String>,
    out_path: String,
    algorithms: Vec<(BlendAlgorithm, Option<BlendAlgorithmParams>)>,
    is_inline: Option<bool>,
    options: Option<Options>,
) -> PyResult<()> {
    let num_images = img_paths.len();

    if num_images < 1 {
        return Err(PyErr::from(PConvertError::ArgumentError(
            "ArgumentError: 'img_paths' must contain at least one path".to_string(),
        )));
    }

    if algorithms.len() != num_images - 1 {
        return Err(PyErr::from(PConvertError::ArgumentError(format!(
            "ArgumentError: 'algorithms' must be of size {} (one per blending operation)",
            num_images - 1
        ))));
    };

    let _is_inline = is_inline.unwrap_or(false);

    // loops through the algorithms to apply and blends the
    // current composition with the next layer
    let mut img_paths_iter = img_paths.iter();
    let first_path = img_paths_iter.next().unwrap().to_string();
    let first_demultiply = is_algorithm_multiplied(&algorithms[0].0);
    let mut composition = read_png_from_file(first_path, first_demultiply)?;
    let zip_iter = img_paths_iter.zip(algorithms.iter());
    for pair in zip_iter {
        let path = pair.0.to_string();
        let (algorithm, algorithm_params) = pair.1;
        let demultiply = is_algorithm_multiplied(&algorithm);
        let algorithm_fn = get_blending_algorithm(&algorithm);
        let current_layer = read_png_from_file(path, demultiply)?;
        blend_images(
            &mut composition,
            &current_layer,
            &algorithm_fn,
            algorithm_params,
        );
    }

    let compression_type = get_compression_type(&options);
    let filter_type = get_filter_type(&options);
    write_png_to_file(out_path, &composition, compression_type, filter_type)?;

    Ok(())
}

unsafe fn blend_multiple_multi_thread(
    img_paths: Vec<String>,
    out_path: String,
    algorithms: Vec<(BlendAlgorithm, Option<BlendAlgorithmParams>)>,
    is_inline: Option<bool>,
    options: Option<Options>,
    num_threads: i32,
) -> PyResult<()> {
    let num_images = img_paths.len();

    if num_images < 1 {
        return Err(PyErr::from(PConvertError::ArgumentError(
            "ArgumentError: 'img_paths' must contain at least one path".to_string(),
        )));
    }

    if algorithms.len() != num_images - 1 {
        return Err(PyErr::from(PConvertError::ArgumentError(format!(
            "ArgumentError: 'algorithms' must be of size {} (one per blending operation)",
            num_images - 1
        ))));
    };

    let _is_inline = is_inline.unwrap_or(false);

    let thread_pool = match &mut THREAD_POOL {
        Some(thread_pool) => thread_pool,
        None => panic!("Unable to access global pconvert thread pool"),
    };

    // expands thread pool to the desired number of threads/parallelism (if necessary and possible)
    thread_pool.expand_to(num_threads as usize);

    let mut png_channels: Vec<mpsc::Receiver<ResultMessage>> = Vec::with_capacity(num_images);
    for path in img_paths.into_iter() {
        let result_channel = thread_pool.execute(move || -> ResultMessage {
            ResultMessage::ImageResult(read_png_from_file(path, false))
        });
        png_channels.push(result_channel);
    }

    let first_demultiply = is_algorithm_multiplied(&algorithms[0].0);
    let mut composition = match png_channels[0].recv().unwrap() {
        ResultMessage::ImageResult(result) => result,
    }?;
    if first_demultiply {
        demultiply_image(&mut composition)
    }

    // loops through the algorithms to apply and blends the
    // current composition with the next layer
    // retrieves the images from the result channels
    for i in 1..png_channels.len() {
        let (algorithm, algorithm_params) = &algorithms[i - 1];
        let demultiply = is_algorithm_multiplied(&algorithm);
        let algorithm_fn = get_blending_algorithm(&algorithm);
        let mut current_layer = match png_channels[i].recv().unwrap() {
            ResultMessage::ImageResult(result) => result,
        }?;
        if demultiply {
            demultiply_image(&mut current_layer)
        }

        blend_images(
            &mut composition,
            &current_layer,
            &algorithm_fn,
            algorithm_params,
        );
    }

    let compression_type = get_compression_type(&options);
    let filter_type = get_filter_type(&options);
    write_png_parallel(out_path, &composition, compression_type, filter_type)?;

    Ok(())
}
