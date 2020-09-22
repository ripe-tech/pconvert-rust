use crate::blending::demultiply_image;
use crate::errors::PConvertError;
use image::io::Reader;
use image::png::{CompressionType, FilterType, PngEncoder};
use image::{ColorType, DynamicImage, ImageBuffer, ImageFormat, Rgba};
use std::fs::File;
use std::io::BufWriter;

pub fn read_png(
    file_in: String,
    demultiply: bool,
) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>, PConvertError> {
    let reader = Reader::open(file_in)?;
    let reader = Reader::with_format(reader.into_inner(), ImageFormat::Png);

    let mut img = match reader.decode() {
        Ok(DynamicImage::ImageRgba8(img)) => img,
        _ => return Err(PConvertError::UnsupportedImageTypeError),
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
) -> Result<(), PConvertError> {
    let file = File::create(&file_out)?;
    let buff = BufWriter::new(file);
    let encoder = PngEncoder::new_with_quality(buff, compression, filter);
    Ok(encoder.encode(&png, png.width(), png.height(), ColorType::Rgba8)?)
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
