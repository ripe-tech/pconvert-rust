mod benchmark;
mod blending;
mod constants;
mod pymodule;
mod utils;

use benchmark::Benchmark;
use blending::{
    blend_images, get_blending_algorithm, is_algorithm_multiplied, multiply_image, Background,
    BlendAlgorithm,
};
use image::png::{CompressionType, FilterType};
use image::{ImageFormat, Rgba};
use std::env;
use std::process;
use std::str;
use std::str::FromStr;
use utils::{read_png, write_png};

pub fn pcompose(args: &mut env::Args) {
    let dir = match args.next() {
        Some(name) => {
            if name.chars().last().unwrap() == '/' {
                name
            } else {
                format!("{}/", name)
            }
        }
        None => {
            println!("Usage: pconvert-rust compose <directory>");
            process::exit(0);
        }
    };

    let mut benchmark = Benchmark::new();

    compose(
        &dir,
        BlendAlgorithm::Alpha,
        Background::Alpha,
        CompressionType::Fast,
        FilterType::NoFilter,
        &mut benchmark,
    );
    compose(
        &dir,
        BlendAlgorithm::Alpha,
        Background::White,
        CompressionType::Fast,
        FilterType::NoFilter,
        &mut benchmark,
    );
    compose(
        &dir,
        BlendAlgorithm::Alpha,
        Background::Blue,
        CompressionType::Fast,
        FilterType::NoFilter,
        &mut benchmark,
    );
    compose(
        &dir,
        BlendAlgorithm::Alpha,
        Background::Texture,
        CompressionType::Fast,
        FilterType::NoFilter,
        &mut benchmark,
    );

    compose(
        &dir,
        BlendAlgorithm::Multiplicative,
        Background::Alpha,
        CompressionType::Fast,
        FilterType::NoFilter,
        &mut benchmark,
    );
    compose(
        &dir,
        BlendAlgorithm::Multiplicative,
        Background::White,
        CompressionType::Fast,
        FilterType::NoFilter,
        &mut benchmark,
    );
    compose(
        &dir,
        BlendAlgorithm::Multiplicative,
        Background::Blue,
        CompressionType::Fast,
        FilterType::NoFilter,
        &mut benchmark,
    );
    compose(
        &dir,
        BlendAlgorithm::Multiplicative,
        Background::Texture,
        CompressionType::Fast,
        FilterType::NoFilter,
        &mut benchmark,
    );

    compose(
        &dir,
        BlendAlgorithm::SourceOver,
        Background::Alpha,
        CompressionType::Fast,
        FilterType::NoFilter,
        &mut benchmark,
    );
    compose(
        &dir,
        BlendAlgorithm::SourceOver,
        Background::White,
        CompressionType::Fast,
        FilterType::NoFilter,
        &mut benchmark,
    );
    compose(
        &dir,
        BlendAlgorithm::SourceOver,
        Background::Blue,
        CompressionType::Fast,
        FilterType::NoFilter,
        &mut benchmark,
    );
    compose(
        &dir,
        BlendAlgorithm::SourceOver,
        Background::Texture,
        CompressionType::Fast,
        FilterType::NoFilter,
        &mut benchmark,
    );

    compose(
        &dir,
        BlendAlgorithm::DestinationOver,
        Background::Alpha,
        CompressionType::Fast,
        FilterType::NoFilter,
        &mut benchmark,
    );
    compose(
        &dir,
        BlendAlgorithm::DestinationOver,
        Background::White,
        CompressionType::Fast,
        FilterType::NoFilter,
        &mut benchmark,
    );
    compose(
        &dir,
        BlendAlgorithm::DestinationOver,
        Background::Blue,
        CompressionType::Fast,
        FilterType::NoFilter,
        &mut benchmark,
    );
    compose(
        &dir,
        BlendAlgorithm::DestinationOver,
        Background::Texture,
        CompressionType::Fast,
        FilterType::NoFilter,
        &mut benchmark,
    );

    compose(
        &dir,
        BlendAlgorithm::FirstTop,
        Background::Alpha,
        CompressionType::Fast,
        FilterType::NoFilter,
        &mut benchmark,
    );
    compose(
        &dir,
        BlendAlgorithm::FirstTop,
        Background::White,
        CompressionType::Fast,
        FilterType::NoFilter,
        &mut benchmark,
    );
    compose(
        &dir,
        BlendAlgorithm::FirstTop,
        Background::Blue,
        CompressionType::Fast,
        FilterType::NoFilter,
        &mut benchmark,
    );
    compose(
        &dir,
        BlendAlgorithm::FirstTop,
        Background::Texture,
        CompressionType::Fast,
        FilterType::NoFilter,
        &mut benchmark,
    );

    compose(
        &dir,
        BlendAlgorithm::FirstBottom,
        Background::Alpha,
        CompressionType::Fast,
        FilterType::NoFilter,
        &mut benchmark,
    );
    compose(
        &dir,
        BlendAlgorithm::FirstBottom,
        Background::White,
        CompressionType::Fast,
        FilterType::NoFilter,
        &mut benchmark,
    );
    compose(
        &dir,
        BlendAlgorithm::FirstBottom,
        Background::Blue,
        CompressionType::Fast,
        FilterType::NoFilter,
        &mut benchmark,
    );
    compose(
        &dir,
        BlendAlgorithm::FirstBottom,
        Background::Texture,
        CompressionType::Fast,
        FilterType::NoFilter,
        &mut benchmark,
    );

    compose(
        &dir,
        BlendAlgorithm::DisjointOver,
        Background::Alpha,
        CompressionType::Fast,
        FilterType::NoFilter,
        &mut benchmark,
    );
    compose(
        &dir,
        BlendAlgorithm::DisjointOver,
        Background::White,
        CompressionType::Fast,
        FilterType::NoFilter,
        &mut benchmark,
    );
    compose(
        &dir,
        BlendAlgorithm::DisjointOver,
        Background::Blue,
        CompressionType::Fast,
        FilterType::NoFilter,
        &mut benchmark,
    );
    compose(
        &dir,
        BlendAlgorithm::DisjointOver,
        Background::Texture,
        CompressionType::Fast,
        FilterType::NoFilter,
        &mut benchmark,
    );

    compose(
        &dir,
        BlendAlgorithm::DisjointUnder,
        Background::Alpha,
        CompressionType::Fast,
        FilterType::NoFilter,
        &mut benchmark,
    );
    compose(
        &dir,
        BlendAlgorithm::DisjointUnder,
        Background::White,
        CompressionType::Fast,
        FilterType::NoFilter,
        &mut benchmark,
    );
    compose(
        &dir,
        BlendAlgorithm::DisjointUnder,
        Background::Blue,
        CompressionType::Fast,
        FilterType::NoFilter,
        &mut benchmark,
    );
    compose(
        &dir,
        BlendAlgorithm::DisjointUnder,
        Background::Texture,
        CompressionType::Fast,
        FilterType::NoFilter,
        &mut benchmark,
    );

    compose(
        &dir,
        BlendAlgorithm::DisjointDebug,
        Background::Alpha,
        CompressionType::Fast,
        FilterType::NoFilter,
        &mut benchmark,
    );
    compose(
        &dir,
        BlendAlgorithm::DisjointDebug,
        Background::White,
        CompressionType::Fast,
        FilterType::NoFilter,
        &mut benchmark,
    );
    compose(
        &dir,
        BlendAlgorithm::DisjointDebug,
        Background::Blue,
        CompressionType::Fast,
        FilterType::NoFilter,
        &mut benchmark,
    );
    compose(
        &dir,
        BlendAlgorithm::DisjointDebug,
        Background::Texture,
        CompressionType::Fast,
        FilterType::NoFilter,
        &mut benchmark,
    );
}

pub fn pconvert(args: &mut env::Args) {
    let file_in = match args.next() {
        Some(name) => name,
        None => {
            println!("Missing input file.\nUsage: pconvert convert <file_in> <file_out>");
            process::exit(0);
        }
    };

    let file_out = match args.next() {
        Some(name) => name,
        None => {
            println!("Missing output path.\nUsage: pconvert convert <file_in> <file_out>");
            process::exit(0);
        }
    };

    let mut img = read_png(file_in, false);

    // turns the image blueish (blue filter)"
    img.pixels_mut().for_each(|x| apply_blue_filter(x));

    img.save_with_format(file_out, ImageFormat::Png)
        .expect("Failure saving modified PNG");
}

pub fn pbenchmark(args: &mut env::Args) {
    let dir = match args.next() {
        Some(name) => {
            if name.chars().last().unwrap() == '/' {
                name
            } else {
                format!("{}/", name)
            }
        }
        None => {
            println!("Usage: pconvert-rust benchmark <directory>");
            process::exit(0);
        }
    };

    // let algorithms = constants::ALGORITHMS;
    // let compressions = [
    //     CompressionType::Default,
    //     CompressionType::Best,
    //     CompressionType::Fast,
    //     CompressionType::Huffman,
    //     CompressionType::Rle,
    // ];
    // let filters = [
    //     FilterType::NoFilter,
    //     FilterType::Avg,
    //     FilterType::Paeth,
    //     FilterType::Sub,
    //     FilterType::Up,
    // ];

    //current pconvert benchmark
    let algorithms = vec![
        "multiplicative",
        "source_over",
        "alpha",
        "disjoint_over",
        "disjoint_under",
    ];
    let compressions = [
        CompressionType::Default,
        CompressionType::Best,
        CompressionType::Fast,
    ];
    let filters = [FilterType::NoFilter];

    println!(
        "{:<20}{:<20}{:<20}{:<20}",
        "Algorithm", "Compression", "Filter", "Times"
    );
    println!("{}", str::from_utf8(&vec![b'-'; 100]).unwrap());

    let mut total_benchmark = Benchmark::new();
    for algorithm in algorithms.iter() {
        for compression in compressions.iter() {
            for filter in filters.iter() {
                let mut benchmark = Benchmark::new();
                compose(
                    &dir,
                    BlendAlgorithm::from_str(&algorithm).unwrap(),
                    Background::Alpha,
                    *compression,
                    *filter,
                    &mut benchmark,
                );
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
) {
    let demultiply = is_algorithm_multiplied(&algorithm);

    let mut bot = benchmark.execute(Benchmark::add_read_png_time, || {
        read_png(format!("{}sole.png", dir), demultiply)
    });

    let algorithm_fn = get_blending_algorithm(&algorithm);

    let top = benchmark.execute(Benchmark::add_read_png_time, || {
        read_png(format!("{}back.png", dir), demultiply)
    });

    benchmark.execute(Benchmark::add_blend_time, || {
        blend_images(&top, &mut bot, &algorithm_fn)
    });

    let top = benchmark.execute(Benchmark::add_read_png_time, || {
        read_png(format!("{}front.png", dir), demultiply)
    });

    benchmark.execute(Benchmark::add_blend_time, || {
        blend_images(&top, &mut bot, &algorithm_fn)
    });

    let top = benchmark.execute(Benchmark::add_read_png_time, || {
        read_png(format!("{}shoelace.png", dir), demultiply)
    });

    benchmark.execute(Benchmark::add_blend_time, || {
        blend_images(&top, &mut bot, &algorithm_fn)
    });

    if demultiply {
        multiply_image(&mut bot)
    }

    let mut composition = read_png(format!("{}background_{}.png", dir, background), false);
    benchmark.execute(Benchmark::add_blend_time, || {
        blend_images(&bot, &mut composition, &algorithm_fn)
    });

    let file_out = format!(
        "{}result_{}_{}_{:#?}_{:#?}.png",
        dir, algorithm, background, compression, filter
    );
    benchmark.execute(Benchmark::add_write_png_time, || {
        write_png(file_out, &composition, compression, filter)
    });
}
