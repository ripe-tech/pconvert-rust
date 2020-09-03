use image::{open, DynamicImage, ImageBuffer, ImageFormat, Rgba};
use std::env;
use std::process;

pub fn pcompose(mut args: env::Args) {
    let _dir = match args.next() {
        Some(name) => name,
        None => {
            println!("Usage: pconvert-rust compose <directory>");
            process::exit(0);
        }
    };
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

    //turn the image blueish: "sets red value to 0 and green value to the blue one (blue filter)"
    img.pixels_mut().for_each(|x| apply_blue_filter(x));

    //save modified PNG
    img.save_with_format(file_out, ImageFormat::Png).expect("Failure saving modified PNG");
}

fn apply_blue_filter(pixel: &mut image::Rgba<u8>) {
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
