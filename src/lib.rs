use image::{open, DynamicImage, ImageFormat};
use std::env;
use std::process;

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

    let mut img = match open(file_in).expect("Failed to open input file") {
        DynamicImage::ImageRgba8(img) => img,
        _ => {
            eprintln!("ERROR: Input file given must be a PNG with RGBA components per pixel");
            process::exit(-1);
        }
    };

    // turns the image blueish: "sets red value to 0 and green value to the blue one (blue filter)"
    img.pixels_mut().for_each(|x| apply_blue_filter(x));

    img.save_with_format(file_out, ImageFormat::Png)
        .expect("Failure saving modified PNG");
}

fn apply_blue_filter(pixel: &mut image::Rgba<u8>) {
    // sets red value to 0 and green value to the blue one (blue filter effect)
    pixel[0] = 0;
    pixel[1] = pixel[2];
}
