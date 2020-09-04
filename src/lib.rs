mod blending;
mod utils;

use image::{ImageFormat, Rgba};
use std::env;
use std::process;

use blending::{
    blend_images, get_blending_algorithm, is_algorithm_multiplied, multiply_image, Background,
    BlendAlgorithm,
};
use utils::read_png;

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

    compose(&dir, BlendAlgorithm::Alpha, Background::Alpha, false);
    compose(&dir, BlendAlgorithm::Alpha, Background::White, false);
    compose(&dir, BlendAlgorithm::Alpha, Background::Blue, false);
    compose(&dir, BlendAlgorithm::Alpha, Background::Texture, false);

    compose(
        &dir,
        BlendAlgorithm::Multiplicative,
        Background::Alpha,
        false,
    );
    compose(
        &dir,
        BlendAlgorithm::Multiplicative,
        Background::White,
        false,
    );
    compose(
        &dir,
        BlendAlgorithm::Multiplicative,
        Background::Blue,
        false,
    );
    compose(
        &dir,
        BlendAlgorithm::Multiplicative,
        Background::Texture,
        false,
    );

    compose(&dir, BlendAlgorithm::SourceOver, Background::Alpha, false);
    compose(&dir, BlendAlgorithm::SourceOver, Background::White, false);
    compose(&dir, BlendAlgorithm::SourceOver, Background::Blue, false);
    compose(&dir, BlendAlgorithm::SourceOver, Background::Texture, false);

    compose(
        &dir,
        BlendAlgorithm::DestinationOver,
        Background::Alpha,
        false,
    );
    compose(
        &dir,
        BlendAlgorithm::DestinationOver,
        Background::White,
        false,
    );
    compose(
        &dir,
        BlendAlgorithm::DestinationOver,
        Background::Blue,
        false,
    );
    compose(
        &dir,
        BlendAlgorithm::DestinationOver,
        Background::Texture,
        false,
    );

    compose(&dir, BlendAlgorithm::FirstTop, Background::Alpha, false);
    compose(&dir, BlendAlgorithm::FirstTop, Background::White, false);
    compose(&dir, BlendAlgorithm::FirstTop, Background::Blue, false);
    compose(&dir, BlendAlgorithm::FirstTop, Background::Texture, false);

    compose(&dir, BlendAlgorithm::FirstBottom, Background::Alpha, false);
    compose(&dir, BlendAlgorithm::FirstBottom, Background::White, false);
    compose(&dir, BlendAlgorithm::FirstBottom, Background::Blue, false);
    compose(
        &dir,
        BlendAlgorithm::FirstBottom,
        Background::Texture,
        false,
    );

    compose(&dir, BlendAlgorithm::DisjointOver, Background::Alpha, false);
    compose(&dir, BlendAlgorithm::DisjointOver, Background::White, false);
    compose(&dir, BlendAlgorithm::DisjointOver, Background::Blue, false);
    compose(
        &dir,
        BlendAlgorithm::DisjointOver,
        Background::Texture,
        false,
    );

    compose(
        &dir,
        BlendAlgorithm::DisjointUnder,
        Background::Alpha,
        false,
    );
    compose(
        &dir,
        BlendAlgorithm::DisjointUnder,
        Background::White,
        false,
    );
    compose(&dir, BlendAlgorithm::DisjointUnder, Background::Blue, false);
    compose(
        &dir,
        BlendAlgorithm::DisjointUnder,
        Background::Texture,
        false,
    );

    compose(
        &dir,
        BlendAlgorithm::DisjointDebug,
        Background::Alpha,
        false,
    );
    compose(
        &dir,
        BlendAlgorithm::DisjointDebug,
        Background::White,
        false,
    );
    compose(&dir, BlendAlgorithm::DisjointDebug, Background::Blue, false);
    compose(
        &dir,
        BlendAlgorithm::DisjointDebug,
        Background::Texture,
        false,
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

    let mut img = read_png(&file_in, false);

    // turns the image blueish (blue filter)"
    img.pixels_mut().for_each(|x| apply_blue_filter(x));

    img.save_with_format(file_out, ImageFormat::Png)
        .expect("Failure saving modified PNG");
}

fn apply_blue_filter(pixel: &mut Rgba<u8>) {
    // sets red value to 0 and green value to the blue one (blue filter effect)
    pixel[0] = 0;
    pixel[1] = pixel[2];
}

fn compose(dir: &str, algorithm: BlendAlgorithm, background: Background, use_opencl: bool) {
    let demultiply = is_algorithm_multiplied(&algorithm);

    let mut bot = read_png(&format!("{}sole.png", dir), demultiply);

    let algorithm_fn = get_blending_algorithm(&algorithm);
    let top = read_png(&format!("{}back.png", dir), demultiply);
    blend_images(&top, &mut bot, &algorithm_fn);

    let top = read_png(&format!("{}front.png", dir), demultiply);
    blend_images(&top, &mut bot, &algorithm_fn);

    let top = read_png(&format!("{}shoelace.png", dir), demultiply);
    blend_images(&top, &mut bot, &algorithm_fn);

    if demultiply {
        multiply_image(&mut bot)
    }

    let top = read_png(&format!("{}background_{}.png", dir, background), false);
    blend_images(&top, &mut bot, &algorithm_fn);

    let file_out = format!(
        "result_{}_{}_{}.png",
        algorithm,
        background,
        if use_opencl { "opencl" } else { "cpu" }
    );

    match bot.save_with_format(format!("{}{}", dir, file_out), ImageFormat::Png) {
        Ok(_) => println!("Successfully composed {}", file_out),
        Err(err) => eprintln!("{}", err),
    }
}
