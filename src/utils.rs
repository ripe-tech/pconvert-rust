use super::blending::demultiply_image;
use image::png::{CompressionType, FilterType, PngEncoder};
use image::ColorType;
use image::{open, DynamicImage, ImageBuffer, Rgba};
use std::fs::File;
use std::io::BufWriter;

pub fn read_png(
    file_in: String,
    demultiply: bool,
) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>, &'static str> {
    let mut img = match open(&file_in) {
        Ok(file) => match file {
            DynamicImage::ImageRgba8(img) => img,
            _ => return Err("Unsupported type: PNG files must be RGBA8"),
        },
        Err(err) => return Err(&err.to_string()),
    };

    if demultiply {
        demultiply_image(&mut img)
    }

    Ok(img)
}

pub fn write_png(
    file_out: String,
    png: &ImageBuffer<Rgba<u8>, Vec<u8>>,
    compression: CompressionType,
    filter: FilterType,
) {
    let file = File::create(&file_out).unwrap();
    let buff = BufWriter::new(file);
    let encoder = PngEncoder::new_with_quality(buff, compression, filter);
    match encoder.encode(&png, png.width(), png.height(), ColorType::Rgba8) {
        Ok(_) => (),
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
