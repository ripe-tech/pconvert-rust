use crate::{
    benchmark::Benchmark,
    blending::BlendAlgorithm,
    compose::{apply_blue_filter, compose, compose_parallel, Background},
};
use crate::{constants, utils::read_png_from_file};
use image::png::{CompressionType, FilterType};
use image::ImageFormat;
use std::str::FromStr;

const TEST_DIR: &str = "assets/test/";
const TEST_FILE: &str = "tux.png";
const TEST_FILE_OUT: &str = "result_tux.png";

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
                BlendAlgorithm::from_str(algorithm).unwrap(),
                background,
                CompressionType::Fast,
                FilterType::NoFilter,
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
                &background,
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
