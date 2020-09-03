use image::{open, DynamicImage, ImageBuffer, Rgba};
use std::process;

pub fn read_png(file_in: &str) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    match open(&file_in) {
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
    }
}

pub fn max(x: f32, y: f32) -> f32 {
    x.max(y)
}

pub fn min(x: f32, y: f32) -> f32 {
    x.min(y)
}
