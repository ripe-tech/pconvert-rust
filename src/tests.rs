use crate::benchmark::Benchmark;
use crate::blending::{
    blend_images, get_blending_algorithm, is_algorithm_multiplied, multiply_image, BlendAlgorithm,
};
use crate::errors::PConvertError;
use crate::parallelism::{ResultMessage, ThreadPool};
use crate::utils::{read_png_from_file, write_png_parallel, write_png_to_file};
use image::codecs::png::{CompressionType, FilterType};
use image::Rgba;
use std::fmt;
use std::fmt::{Display, Formatter};

#[cfg(test)]
use crate::constants;
#[cfg(test)]
use image::ImageFormat;
#[cfg(test)]
use std::str::FromStr;

#[test]
fn test_compose() {
    let test_dir = "assets/test/";
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
                &test_dir,
                BlendAlgorithm::from_str(algorithm).unwrap(),
                background.clone(),
                CompressionType::Fast,
                FilterType::NoFilter,
                &mut benchmark,
            )
            .expect(&format!(
                "failed composing with algorithm={} background={} compression=Fast filter=NoFilter",
                algorithm, background
            ));
        }
    }
}

#[test]
fn test_compose_parallel() {
    let test_dir = "assets/test/";
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
                &test_dir,
                BlendAlgorithm::from_str(algorithm).unwrap(),
                background.clone(),
                CompressionType::Fast,
                FilterType::NoFilter,
                &mut benchmark,
            )
            .expect(&format!(
                "failed composing with algorithm={} background={} compression=Fast filter=NoFilter",
                algorithm, background
            ));
        }
    }
}

#[test]
fn test_convert() {
    let test_dir = "assets/test/";
    let test_file = "tux.png";
    let test_file_out = "result_tux.png";

    let file_in = format!("{}{}", test_dir, test_file);
    let mut img =
        read_png_from_file(file_in.clone(), false).expect(&format!("failure reading {}", file_in));

    for pixel in img.pixels_mut() {
        apply_blue_filter(pixel);
    }

    let out = format!("{}{}", test_dir, test_file_out);
    img.save_with_format(out.clone(), ImageFormat::Png)
        .expect(&format!("failure writing {}", out));
}

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

pub fn compose(
    dir: &str,
    algorithm: BlendAlgorithm,
    background: Background,
    compression: CompressionType,
    filter: FilterType,
    benchmark: &mut Benchmark,
) -> Result<String, PConvertError> {
    let demultiply = is_algorithm_multiplied(&algorithm);

    let mut bot = benchmark.execute(Benchmark::add_read_png_time, || {
        read_png_from_file(format!("{}sole.png", dir), demultiply)
    })?;

    let algorithm_fn = get_blending_algorithm(&algorithm);

    // reads one PNG at the time and blends it with the current result

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

    // writes the final composition to the file system
    let file_name = format!(
        "result_{}_{}_{:#?}_{:#?}.png",
        algorithm, background, compression, filter
    );
    let file_out = format!("{}{}", dir, file_name.clone());
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
    let file_name = format!(
        "result_{}_{}_{:#?}_{:#?}.png",
        algorithm, background, compression, filter
    );
    let file_out = format!("{}{}", dir, file_name.clone());
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
