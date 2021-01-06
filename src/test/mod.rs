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

#[cfg(test)]
use crate::constants;
#[cfg(test)]
use image::ImageFormat;
#[cfg(test)]
use std::str::FromStr;

const THREAD_POOL_SIZE: usize = 5;

#[cfg(test)]
const TEST_DIR: &str = "assets/test/";
#[cfg(test)]
const TEST_FILE: &str = "tux.png";
#[cfg(test)]
const TEST_FILE_OUT: &str = "result_tux.png";

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

#[test]
fn test_benchmark() {
    let mut benchmark1 = Benchmark::new();
    Benchmark::add_blend_time(&mut benchmark1, 100);
    Benchmark::add_read_png_time(&mut benchmark1, 200);
    Benchmark::add_write_png_time(&mut benchmark1, 150);

    assert!(benchmark1.total() == 100 + 200 + 150);

    let mut benchmark2 = Benchmark::new();
    Benchmark::add_blend_time(&mut benchmark2, 50);
    Benchmark::add_read_png_time(&mut benchmark2, 100);
    Benchmark::add_write_png_time(&mut benchmark2, 75);

    assert!(benchmark2.total() == 50 + 100 + 75);

    let sum_benchmark = benchmark1 + benchmark2;
    assert!(sum_benchmark.total() == 100 + 200 + 150 + 50 + 100 + 75);
}

#[test]
fn test_compose() {
    let mut benchmark = Benchmark::new();
    let backgrounds = vec![
        Background::Alpha,
        Background::Blue,
        Background::Texture,
        Background::White,
    ];

    // composes with different combinations of blending algorithms and backgrounds
    for background in &backgrounds {
        for algorithm in constants::ALGORITHMS.iter() {
            compose(
                &TEST_DIR,
                &BlendAlgorithm::from_str(algorithm).unwrap(),
                background,
                &CompressionType::Fast,
                &FilterType::NoFilter,
                &mut benchmark,
            )
            .unwrap_or_else(|_| panic!("failed composing with algorithm={} background={} compression=Fast filter=NoFilter", algorithm, background));
        }
    }
}

#[test]
fn test_compose_parallel() {
    let mut benchmark = Benchmark::new();
    let backgrounds = vec![
        Background::Alpha,
        Background::Blue,
        Background::Texture,
        Background::White,
    ];

    // composes with different combinations of blending algorithms and backgrounds
    for background in backgrounds {
        for algorithm in constants::ALGORITHMS.iter() {
            compose_parallel(
                &TEST_DIR,
                BlendAlgorithm::from_str(algorithm).unwrap(),
                background.clone(),
                CompressionType::Fast,
                FilterType::NoFilter,
                &mut benchmark,
            )
            .unwrap_or_else(|_| panic!("failed composing with algorithm={} background={} compression=Fast filter=NoFilter", algorithm, background));
        }
    }
}

#[test]
fn test_convert() {
    let file_in = format!("{}{}", TEST_DIR, TEST_FILE);
    let mut img = read_png_from_file(file_in.clone(), false)
        .unwrap_or_else(|_| panic!("failure reading {}", file_in));

    for pixel in img.pixels_mut() {
        apply_blue_filter(pixel);
    }

    let out = format!("{}{}", TEST_DIR, TEST_FILE_OUT);
    img.save_with_format(out.clone(), ImageFormat::Png)
        .unwrap_or_else(|_| panic!("failure writing {}", out));
}

pub fn compose(
    dir: &str,
    algorithm: &BlendAlgorithm,
    background: &Background,
    compression: &CompressionType,
    filter: &FilterType,
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

pub fn compose_parallel(
    dir: &str,
    algorithm: BlendAlgorithm,
    background: Background,
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

    for i in 1..=result_channels.len() - 2 {
        let top = benchmark.execute(Benchmark::add_read_png_time, || {
            if let Ok(ResultMessage::ImageResult(result)) = result_channels[i].recv() {
                result
            } else {
                panic!(format!("failure reading '{}'", png_file_names[i]))
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
            panic!(format!("failure reading '{}'", background_file))
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

pub fn apply_blue_filter(pixel: &mut Rgba<u8>) {
    // sets red value to 0 and green value to the blue one (blue filter effect)
    pixel[0] = 0;
    pixel[1] = pixel[2];
}
