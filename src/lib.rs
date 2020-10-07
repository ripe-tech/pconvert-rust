mod benchmark;
pub mod blending;
mod constants;
pub mod errors;
mod parallelism;
mod utils;

#[cfg(not(target_arch = "wasm32"))]
mod pymodule;

#[cfg(target_arch = "wasm32")]
mod wasm;

use benchmark::Benchmark;
use blending::{
    blend_images, get_blending_algorithm, is_algorithm_multiplied, multiply_image, Background,
    BlendAlgorithm,
};
use errors::PConvertError;
use image::png::{CompressionType, FilterType};
use image::{ImageFormat, Rgba};
use parallelism::{ResultMessage, ThreadPool};
use std::env;
use std::str;
use std::str::FromStr;
use utils::{read_png_from_file, write_png_to_file};

#[cfg(not(target_arch = "wasm32"))]
use utils::write_png_parallel;

pub fn pcompose(args: &mut env::Args) -> Result<(), PConvertError> {
    let dir = match args.next() {
        Some(name) => {
            if name.chars().last().unwrap() == '/' {
                name
            } else {
                format!("{}/", name)
            }
        }
        None => {
            return Err(PConvertError::ArgumentError(
                "ArgumentError: 'directory' not specified".to_string(),
            ))
        }
    };

    let mut benchmark = Benchmark::new();

    let backgrounds = vec![
        Background::Alpha,
        Background::Blue,
        Background::Texture,
        Background::White,
    ];
    for background in backgrounds {
        for algorithm in constants::ALGORITHMS.iter() {
            compose(
                &dir,
                BlendAlgorithm::from_str(algorithm).unwrap(),
                background.clone(),
                CompressionType::Fast,
                FilterType::NoFilter,
                &mut benchmark,
            )?;
        }
    }

    Ok(())
}

pub fn pconvert(args: &mut env::Args) -> Result<(), PConvertError> {
    let file_in = match args.next() {
        Some(name) => name,
        None => {
            return Err(PConvertError::ArgumentError(
                "ArgumentError: 'file_in' not specified".to_string(),
            ))
        }
    };

    let file_out = match args.next() {
        Some(name) => name,
        None => {
            return Err(PConvertError::ArgumentError(
                "ArgumentError: 'file_out' not specified".to_string(),
            ))
        }
    };

    let mut img = read_png_from_file(file_in, false)?;

    for pixel in img.pixels_mut() {
        apply_blue_filter(pixel);
    }

    img.save_with_format(file_out, ImageFormat::Png)?;

    Ok(())
}

pub fn pbenchmark(args: &mut env::Args) -> Result<(), PConvertError> {
    let dir = match args.next() {
        Some(name) => {
            if name.chars().last().unwrap() == '/' {
                name
            } else {
                format!("{}/", name)
            }
        }
        None => {
            return Err(PConvertError::ArgumentError(
                "ArgumentError: 'directory' not specified".to_string(),
            ))
        }
    };

    let run_parallel = match args.next() {
        Some(flag) => flag.eq("--parallel"),
        _ => false,
    };

    println!(
        "{:<20}{:<20}{:<20}{:<20}",
        "Algorithm", "Compression", "Filter", "Times"
    );
    println!("{}", str::from_utf8(&vec![b'-'; 100]).unwrap());

    let mut total_benchmark = Benchmark::new();
    for algorithm in constants::ALGORITHMS.iter() {
        for compression in constants::COMPRESSION_TYPES.iter() {
            for filter in constants::FILTER_TYPES.iter() {
                let mut benchmark = Benchmark::new();
                if run_parallel {
                    compose_parallel(
                        &dir,
                        BlendAlgorithm::from_str(&algorithm).unwrap(),
                        Background::Alpha,
                        *compression,
                        *filter,
                        &mut benchmark,
                    )?;
                } else {
                    compose(
                        &dir,
                        BlendAlgorithm::from_str(&algorithm).unwrap(),
                        Background::Alpha,
                        *compression,
                        *filter,
                        &mut benchmark,
                    )?;
                }

                println!(
                    "{:<20}{:<20}{:<20}{:<20}",
                    algorithm,
                    format!("{:#?}", compression),
                    format!("{:#?}", filter),
                    &benchmark
                );
                total_benchmark = total_benchmark + benchmark;
            }
        }
        println!();
    }
    println!("\nTotal time: {:<20}", &total_benchmark);
    Ok(())
}

pub fn pversion() -> Result<(), PConvertError> {
    println!(
        "P(NG)Convert Rust {} ({} {}) [{} {} {} bit] [libpng {}] {:?}",
        constants::VERSION,
        constants::COMPILATION_DATE,
        constants::COMPILATION_TIME,
        constants::COMPILER,
        constants::COMPILER_VERSION,
        constants::PLATFORM_CPU_BITS,
        constants::LIBPNG_VERSION,
        constants::FEATURES
    );
    println!("Copyright (c) 2008-2020 Platforme International Limited. All rights reserved.");
    Ok(())
}

fn apply_blue_filter(pixel: &mut Rgba<u8>) {
    // sets red value to 0 and green value to the blue one (blue filter effect)
    pixel[0] = 0;
    pixel[1] = pixel[2];
}

fn compose(
    dir: &str,
    algorithm: BlendAlgorithm,
    background: Background,
    compression: CompressionType,
    filter: FilterType,
    benchmark: &mut Benchmark,
) -> Result<(), PConvertError> {
    let demultiply = is_algorithm_multiplied(&algorithm);

    let mut bot = benchmark.execute(Benchmark::add_read_png_time, || {
        read_png_from_file(format!("{}sole.png", dir), demultiply)
    })?;

    let algorithm_fn = get_blending_algorithm(&algorithm);

    let top = benchmark.execute(Benchmark::add_read_png_time, || {
        read_png_from_file(format!("{}back.png", dir), demultiply)
    })?;

    benchmark.execute(Benchmark::add_blend_time, || {
        blend_images(&top, &mut bot, &algorithm_fn, &None)
    });

    let top = benchmark.execute(Benchmark::add_read_png_time, || {
        read_png_from_file(format!("{}front.png", dir), demultiply)
    })?;

    benchmark.execute(Benchmark::add_blend_time, || {
        blend_images(&top, &mut bot, &algorithm_fn, &None)
    });

    let top = benchmark.execute(Benchmark::add_read_png_time, || {
        read_png_from_file(format!("{}shoelace.png", dir), demultiply)
    })?;

    benchmark.execute(Benchmark::add_blend_time, || {
        blend_images(&top, &mut bot, &algorithm_fn, &None)
    });

    if demultiply {
        benchmark.execute(Benchmark::add_blend_time, || multiply_image(&mut bot));
    }

    let mut composition = benchmark.execute(Benchmark::add_read_png_time, || {
        read_png_from_file(format!("{}background_{}.png", dir, background), false)
    })?;

    benchmark.execute(Benchmark::add_blend_time, || {
        blend_images(&bot, &mut composition, &algorithm_fn, &None)
    });

    let file_out = format!(
        "{}result_{}_{}_{:#?}_{:#?}.png",
        dir, algorithm, background, compression, filter
    );
    benchmark.execute(Benchmark::add_write_png_time, || {
        write_png_to_file(file_out, &composition, compression, filter)
    })?;

    Ok(())
}

fn compose_parallel(
    dir: &str,
    algorithm: BlendAlgorithm,
    background: Background,
    compression: CompressionType,
    filter: FilterType,
    benchmark: &mut Benchmark,
) -> Result<(), PConvertError> {
    let demultiply = is_algorithm_multiplied(&algorithm);
    let algorithm_fn = get_blending_algorithm(&algorithm);

    let mut thread_pool = ThreadPool::new(5)?;

    // sends the PNG reading tasks to multiple threads
    // these values are hardcoded by the multiple layer files
    let png_file_names = vec![
        "sole.png".to_owned(),
        "back.png".to_owned(),
        "front.png".to_owned(),
        "shoelace.png".to_owned(),
        format!("background_{}.png", background),
    ];

    let mut result_channels = Vec::with_capacity(png_file_names.len());
    thread_pool.start();
    for png_file_name in png_file_names {
        let path = format!("{}{}", dir, png_file_name);
        let result_channel = thread_pool
            .execute(move || ResultMessage::ImageResult(read_png_from_file(path, demultiply)));
        result_channels.push(result_channel);
    }

    // blending phase, will run the multiple layers operation
    // as expected by the proper execution
    let mut bot = benchmark.execute(Benchmark::add_read_png_time, || {
        match result_channels[0].recv().unwrap() {
            ResultMessage::ImageResult(result) => result,
        }
    })?;

    let top = benchmark.execute(Benchmark::add_read_png_time, || {
        match result_channels[1].recv().unwrap() {
            ResultMessage::ImageResult(result) => result,
        }
    })?;
    benchmark.execute(Benchmark::add_blend_time, || {
        blend_images(&top, &mut bot, &algorithm_fn, &None)
    });

    let top = benchmark.execute(Benchmark::add_read_png_time, || {
        match result_channels[2].recv().unwrap() {
            ResultMessage::ImageResult(result) => result,
        }
    })?;
    benchmark.execute(Benchmark::add_blend_time, || {
        blend_images(&top, &mut bot, &algorithm_fn, &None)
    });

    let top = benchmark.execute(Benchmark::add_read_png_time, || {
        match result_channels[3].recv().unwrap() {
            ResultMessage::ImageResult(result) => result,
        }
    })?;
    benchmark.execute(Benchmark::add_blend_time, || {
        blend_images(&top, &mut bot, &algorithm_fn, &None)
    });

    if demultiply {
        multiply_image(&mut bot);
    }

    let mut composition =
        benchmark.execute(Benchmark::add_read_png_time, || {
            match result_channels[4].recv().unwrap() {
                ResultMessage::ImageResult(result) => result,
            }
        })?;
    benchmark.execute(Benchmark::add_blend_time, || {
        blend_images(&bot, &mut composition, &algorithm_fn, &None)
    });

    // writes the final composition PNG to the output file,
    // this is considered to be the most expensive operation
    let file_out = format!(
        "{}result_{}_{}_{:#?}_{:#?}.png",
        dir, algorithm, background, compression, filter
    );
    benchmark.execute(Benchmark::add_write_png_time, || {
        #[cfg(target_arch = "wasm32")]
        {
            println!("Warning: running on WASM32 target, parallel PNG writing using MTPNG crate is not allowed.");
            println!("Using single-threaded write_png_to_file");
            write_png_to_file(file_out, &composition, compression, filter)
        }

        #[cfg(not(target_arch = "wasm32"))] 
        {
            write_png_parallel(file_out, &composition, compression, filter)
        }
    })?;

    Ok(())
}
