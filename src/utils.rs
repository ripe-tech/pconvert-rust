use crate::blending::demultiply_image;
use crate::errors::PConvertError;
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
    compression: CompressionType,
    filter: FilterType,
) -> Result<(), PConvertError> {
    let writer = File::create(file_out)?;

    let mut header = mtpng::Header::new();
    header.set_size(png.width(), png.height())?;
    header.set_color(mtpng::ColorType::TruecolorAlpha, 8)?;

    let mut options = mtpng::encoder::Options::new();
    options.set_compression_level(mtpng_compression_from(compression))?;
    options.set_filter_mode(mtpng::Mode::Fixed(mtpng_filter_from(filter)))?;

    let mut encoder = mtpng::encoder::Encoder::new(writer, &options);
    encoder.write_header(&header)?;
    encoder.write_image_rows(&png)?;
    encoder.finish()?;

    Ok(())
}

pub fn image_compression_from(compression: String) -> CompressionType {
    match compression.trim().to_lowercase().as_str() {
        "best" => CompressionType::Best,
        "default" => CompressionType::Default,
        "fast" => CompressionType::Fast,
        "huffman" => CompressionType::Huffman,
        "rle" => CompressionType::Rle,
        _ => CompressionType::Fast,
    }
}

pub fn image_filter_from(filter: String) -> FilterType {
    match filter.trim().to_lowercase().as_str() {
        "avg" => FilterType::Avg,
        "nofilter" => FilterType::NoFilter,
        "paeth" => FilterType::Paeth,
        "sub" => FilterType::Sub,
        "up" => FilterType::Up,
        _ => FilterType::NoFilter,
    }
}

fn mtpng_compression_from(compression: CompressionType) -> mtpng::CompressionLevel {
    match compression {
        CompressionType::Default => mtpng::CompressionLevel::Default,
        CompressionType::Best => mtpng::CompressionLevel::High,
        CompressionType::Fast => mtpng::CompressionLevel::Fast,
        _ => mtpng::CompressionLevel::Fast,
    }
}

fn mtpng_filter_from(filter: FilterType) -> mtpng::Filter {
    match filter {
        FilterType::Avg => mtpng::Filter::Average,
        FilterType::Paeth => mtpng::Filter::Paeth,
        FilterType::Sub => mtpng::Filter::Sub,
        FilterType::Up => mtpng::Filter::Up,
        FilterType::NoFilter => mtpng::Filter::None,
        _ => mtpng::Filter::None,
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
