use super::blending::demultiply_image;
use super::errors::PConvertError;
use image::io::Reader;
use image::png::{CompressionType, FilterType, PngEncoder};
use image::{ColorType, DynamicImage, ImageBuffer, ImageFormat, Rgba};
use mtpng;
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

pub fn write_png_parallel(
    file_out: String,
    png: &ImageBuffer<Rgba<u8>, Vec<u8>>,
    _compression: CompressionType,
    _filter: FilterType,
) -> Result<(), PConvertError> {
    let writer = File::create(file_out)?;

    let mut header = mtpng::Header::new();
    header.set_size(png.width(), png.height())?;
    header.set_color(mtpng::ColorType::TruecolorAlpha, 8)?;

    let options = mtpng::encoder::Options::new();
    // options.set_compression_level(level: CompressionLevel)
    // options.set_filter_mode(filter_mode: Mode<Filter>)

    let mut encoder = mtpng::encoder::Encoder::new(writer, &options);

    encoder.write_header(&header)?;
    encoder.write_image_rows(&png)?;
    encoder.finish()?;

    Ok(())
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
