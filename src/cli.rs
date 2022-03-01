use image::codecs::png::{CompressionType, FilterType};
use image::ImageFormat;
use pconvert_rust::benchmark::Benchmark;
use pconvert_rust::blending::BlendAlgorithm;
use pconvert_rust::compose::{apply_blue_filter, compose, compose_parallel, Background};
use pconvert_rust::constants;
use pconvert_rust::errors::PConvertError;
use pconvert_rust::utils::read_png_from_file;
use std::env;
use std::str;
use std::str::FromStr;

pub fn print_usage() {
    println!("Usage: pconvert-rust <command> [args...]\nwhere command can be one of the following: compose, convert, benchmark, version");
}

pub fn pcompose(args: &mut env::Args) -> Result<(), PConvertError> {
    let dir = match args.next() {
        Some(name) => {
            if name.ends_with('/') {
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

    // composes with different combinations of blending algorithms and backgrounds
    for background in backgrounds {
        for algorithm in constants::ALGORITHMS.iter() {
            compose(
                &dir,
                BlendAlgorithm::from_str(algorithm).unwrap(),
                &background,
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
            if name.ends_with('/') {
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
    println!("{}", str::from_utf8(&[b'-'; 100]).unwrap());

    // tries and outputs to stdout times for different combinations of
    // blending algorithms, compression and filter types
    let mut total_benchmark = Benchmark::new();
    for algorithm in constants::ALGORITHMS.iter() {
        for compression in constants::COMPRESSION_TYPES.iter() {
            for filter in constants::FILTER_TYPES.iter() {
                let mut benchmark = Benchmark::new();
                if run_parallel {
                    compose_parallel(
                        &dir,
                        BlendAlgorithm::from_str(&algorithm).unwrap(),
                        &Background::Alpha,
                        *compression,
                        *filter,
                        &mut benchmark,
                    )?;
                } else {
                    compose(
                        &dir,
                        BlendAlgorithm::from_str(&algorithm).unwrap(),
                        &Background::Alpha,
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

pub fn pversion() {
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
    println!("Copyright (c) 2008-2022 Platforme International Limited. All rights reserved.");
}
