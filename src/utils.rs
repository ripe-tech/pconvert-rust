use super::blending::demultiply_image;
use image::png::{CompressionType, FilterType, PngEncoder};
use image::ColorType;
use image::{open, DynamicImage, ImageBuffer, Rgba};
use std::fs::File;
use std::io::BufWriter;
use std::process;

pub fn read_png(file_in: String, demultiply: bool) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
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

pub fn write_png(file_out: String, png: &ImageBuffer<Rgba<u8>, Vec<u8>>) {
    let file = File::create(&file_out).unwrap();
    let buff = BufWriter::new(file);
    let encoder = PngEncoder::new_with_quality(buff, CompressionType::Fast, FilterType::NoFilter);
    match encoder.encode(&png, png.width(), png.height(), ColorType::Rgba8) {
        Ok(_) => println!("Successfully saved {}", &file_out),
        Err(_) => eprintln!("ERROR: writing {}", &file_out),
    }
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
