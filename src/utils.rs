use super::blending::demultiply_image;
use image::{open, DynamicImage, ImageBuffer, Rgba};
use std::process;

pub fn read_png(file_in: &str, demultiply: bool) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let mut img = match open(&file_in) {
        Ok(file) => match file {
            DynamicImage::ImageRgba8(img) => img,
            _ => {
                eprintln!("ERROR: Specified input file must be PNG-RGBA where each component is one byte (RGBA8)");
                process::exit(-1);
            }
        },
        Err(_) => {
            eprintln!("ERROR: Failure opening file {}", &file_in);
            process::exit(-1);
        }
    };

    if demultiply {
        demultiply_image(&mut img)
    }
    img
}

pub fn max<T: PartialOrd>(x: T, y: T) -> T {
    if x > y {
        x
    } else {
        y
    }
}

pub fn min<T: PartialOrd>(x: T, y: T) -> T {
    if x < y {
        x
    } else {
        y
    }
}
