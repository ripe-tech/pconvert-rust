use image::{open, DynamicImage, ImageBuffer, ImageFormat, Rgba};
use std::env;
use std::process;

pub fn pcompose(mut args: env::Args) {
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

    let mut bot = read_png(format!("{}sole.png", dir));
    let top = read_png(format!("{}back.png", dir));

    blend_images(&top, &mut bot, blend_alpha);

    bot.save_with_format(format!("{}resultcomposition.png", dir), ImageFormat::Png)
        .expect("Failure saving composition");
}

pub fn pconvert(mut args: env::Args) {
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

    //read PNG
    let mut img = read_png(file_in);

    //turn the image blueish (blue filter)"
    img.pixels_mut().for_each(|x| apply_blue_filter(x));

    //save modified PNG
    img.save_with_format(file_out, ImageFormat::Png)
        .expect("Failure saving modified PNG");
}

fn apply_blue_filter(pixel: &mut Rgba<u8>) {
    // sets red value to 0 and green value to the blue one (blue filter effect)
    pixel[0] = 0;
    pixel[1] = pixel[2];
}

fn read_png(file_in: String) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    match open(&file_in) {
        Ok(file) => match file {
            DynamicImage::ImageRgba8(img) => img,
            _ => {
                eprintln!("ERROR: Specified input file must be PNG-RGBA");
                process::exit(-1);
            }
        },
        Err(_) => {
            eprintln!("ERROR: Failure opening file {}", &file_in);
            process::exit(-1);
        }
    }
}

/* Blends 2 PNGs, updating the bottom reference */
fn blend_images(
    top: &ImageBuffer<Rgba<u8>, Vec<u8>>,
    bot: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
    blending_algorithm: impl Fn((&mut Rgba<u8>, &Rgba<u8>)) -> (),
) {
    bot.pixels_mut()
        .zip(top.pixels())
        .for_each(|x| blending_algorithm(x));
}

fn blend_alpha((bot_pixel, top_pixel): (&mut Rgba<u8>, &Rgba<u8>)) {
    let (rb, gb, bb, ab) = (bot_pixel[0], bot_pixel[1], bot_pixel[2], bot_pixel[3]);
    let (rt, gt, bt, at) = (top_pixel[0], top_pixel[1], top_pixel[2], top_pixel[3]);

    let abf = 1.0 * (ab as f32 / 255.0);
    let atf = 1.0 * (at as f32 / 255.0);
    let af = atf + abf * (1.0 - atf);

    let r = if af == 0.0 {
        0.0
    } else {
        (rb as f32 * abf + rt as f32 * atf * (1.0 - abf)) / af
    };
    let g = if af == 0.0 {
        0.0
    } else {
        (gb as f32 * abf + gt as f32 * atf * (1.0 - abf)) / af
    };
    let b = if af == 0.0 {
        0.0
    } else {
        (bb as f32 * abf + bt as f32 * atf * (1.0 - abf)) / af
    };
    let a = max(0.0, min(255.0, (abf + atf * (1.0 - abf)) * 255.0));

    let r = max(0.0, min(255.0, r));
    let g = max(0.0, min(255.0, g));
    let b = max(0.0, min(255.0, b));

    bot_pixel[0] = r as u8;
    bot_pixel[1] = g as u8;
    bot_pixel[2] = b as u8;
    bot_pixel[3] = a as u8;
}

fn max(x: f32, y: f32) -> f32 {
    x.max(y)
}

fn min(x: f32, y: f32) -> f32 {
    x.min(y)
}
