use crate::benchmark::Benchmark;
use crate::blending::{
    blend_images, get_blending_algorithm, is_algorithm_multiplied, multiply_image, BlendAlgorithm,
};
use crate::errors::PConvertError;
use crate::parallelism::{ResultMessage, ThreadPool};
use crate::utils::{read_png_from_file, write_png_parallel, write_png_to_file};
use image::codecs::png::{CompressionType, FilterType};
use image::Rgba;
use std::fmt::{Display, Formatter};
use std::{fmt, sync::mpsc::Receiver};

const THREAD_POOL_SIZE: usize = 5;

#[derive(Clone)]
pub enum Background {
    Alpha,
    White,
    Blue,
    Texture,
}

impl Display for Background {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Background::Alpha => write!(f, "alpha"),
            Background::White => write!(f, "white"),
            Background::Blue => write!(f, "blue"),
            Background::Texture => write!(f, "texture"),
        }
    }
}

/// Testing utility that composes an image made up of the specified
/// background image, using the specified algorithm, compression and filter types.
/// Looks for the layers and outputs the final composition to the given `dir` and
/// takes track of times spent in each phase in the benchmark struct
pub fn compose(
    dir: &str,
    algorithm: BlendAlgorithm,
    background: &Background,
    compression: CompressionType,
    filter: FilterType,
    benchmark: &mut Benchmark,
) -> Result<String, PConvertError> {
    let demultiply = is_algorithm_multiplied(&algorithm);

    let algorithm_fn = get_blending_algorithm(&algorithm);

    // reads one PNG at the time and blends it with the current result
    // these values are hardcoded by the multiple layer files
    let background_file = format!("background_{}.png", background);
    let png_file_names = vec![
        "sole.png",
        "back.png",
        "front.png",
        "shoelace.png",
        &background_file,
    ];

    let png_paths = png_file_names
        .iter()
        .map(|name| format!("{}{}", dir, name))
        .collect::<Vec<String>>();

    let top = benchmark.execute(Benchmark::add_read_png_time, || {
        read_png_from_file(format!("{}sole.png", dir), demultiply)
    })?;

    let mut bot =
        png_paths[..png_file_names.len() - 1]
            .iter()
            .fold(top, |mut composition, path| {
                let layer = benchmark
                    .execute(Benchmark::add_read_png_time, || {
                        read_png_from_file(path.clone(), demultiply)
                    })
                    .unwrap();

                benchmark.execute(Benchmark::add_blend_time, || {
                    blend_images(&mut composition, &layer, &algorithm_fn, &None)
                });

                composition
            });

    if demultiply {
        benchmark.execute(Benchmark::add_blend_time, || multiply_image(&mut bot));
    }

    let mut composition = benchmark.execute(Benchmark::add_read_png_time, || {
        read_png_from_file(format!("{}background_{}.png", dir, background), false)
    })?;

    benchmark.execute(Benchmark::add_blend_time, || {
        blend_images(&mut composition, &bot, &algorithm_fn, &None)
    });

    // writes the final composition to the file system
    let file_name = format!(
        "result_{}_{}_{:#?}_{:#?}.png",
        algorithm, background, compression, filter
    );
    let file_out = format!("{}{}", dir, file_name);
    benchmark.execute(Benchmark::add_write_png_time, || {
        write_png_to_file(file_out, &composition, compression, filter)
    })?;

    Ok(file_name)
}

/// Multi-threaded version of the `compose` testing utility
/// Reads each PNG in a different thread and makes use of the
/// `mtpng` library to write the final composition
pub fn compose_parallel(
    dir: &str,
    algorithm: BlendAlgorithm,
    background: &Background,
    compression: CompressionType,
    filter: FilterType,
    benchmark: &mut Benchmark,
) -> Result<String, PConvertError> {
    let demultiply = is_algorithm_multiplied(&algorithm);
    let algorithm_fn = get_blending_algorithm(&algorithm);

    let mut thread_pool = ThreadPool::new(THREAD_POOL_SIZE)?;
    thread_pool.start();

    // sends the PNG reading tasks to multiple threads
    // these values are hardcoded by the multiple layer files
    let background_file = format!("background_{}.png", background);
    let png_file_names = vec![
        "sole.png",
        "back.png",
        "front.png",
        "shoelace.png",
        &background_file,
    ];

    let result_channels = png_file_names
        .iter()
        .map(|name| format!("{}{}", dir, name))
        .map(|path| {
            thread_pool
                .execute(move || ResultMessage::ImageResult(read_png_from_file(path, demultiply)))
        })
        .collect::<Vec<Receiver<ResultMessage>>>();

    // blending phase, will run the multiple layers operation
    // as expected by the proper execution
    let mut bot = benchmark.execute(Benchmark::add_read_png_time, || {
        if let Ok(ResultMessage::ImageResult(result)) = result_channels[0].recv() {
            result
        } else {
            panic!("failure reading 'sole.png'")
        }
    })?;

    for i in 1..=3 {
        let top = benchmark.execute(Benchmark::add_read_png_time, || {
            if let Ok(ResultMessage::ImageResult(result)) = result_channels[i].recv() {
                result
            } else {
                panic!("failure reading '{}'", png_file_names[i])
            }
        })?;
        benchmark.execute(Benchmark::add_blend_time, || {
            blend_images(&mut bot, &top, &algorithm_fn, &None)
        });
    }

    if demultiply {
        multiply_image(&mut bot);
    }

    let mut composition = benchmark.execute(Benchmark::add_read_png_time, || {
        if let Ok(ResultMessage::ImageResult(result)) = result_channels[4].recv() {
            result
        } else {
            panic!("failure reading '{}'", background_file)
        }
    })?;
    benchmark.execute(Benchmark::add_blend_time, || {
        blend_images(&mut composition, &bot, &algorithm_fn, &None)
    });

    // writes the final composition PNG to the output file,
    // this is considered to be the most expensive operation
    let file_name = format!(
        "result_{}_{}_{:#?}_{:#?}.png",
        algorithm, background, compression, filter
    );
    let file_out = format!("{}{}", dir, file_name);
    benchmark.execute(Benchmark::add_write_png_time, || {
        write_png_parallel(file_out, &composition, compression, filter)
    })?;

    Ok(file_name)
}

/// Testing utility that applies a blue-ish filter to an image
pub fn apply_blue_filter(pixel: &mut Rgba<u8>) {
    // sets red value to 0 and green value to the blue one (blue filter effect)
    pixel[0] = 0;
    pixel[1] = pixel[2];
}
