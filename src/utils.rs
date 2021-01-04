//! PNG decode/encode and read/write functions, external crate type
//! conversions and other utility functions.

use crate::blending::demultiply_image;
use crate::errors::PConvertError;
use image::codecs::png::{CompressionType, FilterType, PngDecoder, PngEncoder};
use image::ImageDecoder;
use image::{ColorType, ImageBuffer, Rgba};
use std::fs::File;
use std::io::{BufWriter, Read, Write};

/// Decodes and returns a PNG.
///
/// # Arguments
///
/// * `readable_stream` - Any structure that implements the `Read` trait.
/// * `demultiply` - Whether or not to demultiply the PNG.
pub fn decode_png(
    readable_stream: impl Read,
    demultiply: bool,
) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>, PConvertError> {
    let decoder = PngDecoder::new(readable_stream)?;
    let (width, height) = decoder.dimensions();

    let mut reader = decoder.into_reader()?;

    let mut bytes = Vec::<u8>::new();
    reader.read_to_end(&mut bytes)?;

    let mut img = ImageBuffer::from_vec(width, height, bytes).unwrap();

    if demultiply {
        demultiply_image(&mut img)
    }

    Ok(img)
}

/// Reads a PNG from the local file system.
///
/// # Arguments
///
/// * `file_in` - Local file system path to the PNG file.
/// * `demultiply` - Whether or not to demultiply the PNG.
pub fn read_png_from_file(
    file_in: String,
    demultiply: bool,
) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>, PConvertError> {
    let file = File::open(file_in)?;
    decode_png(file, demultiply)
}

/// Encodes a PNG and writes it to a buffer.
///
/// # Arguments
///
/// * `writable_buff` - Any buffer structure that implements the `Write` trait.
/// * `png` - A byte buffer with the image data.
/// * `compression` - Compression type to use in the encoding.
/// * `filter` - Filter type to use in the encoding.
pub fn encode_png(
    writable_buff: impl Write,
    png: &ImageBuffer<Rgba<u8>, Vec<u8>>,
    compression: CompressionType,
    filter: FilterType,
) -> Result<(), PConvertError> {
    let buff = BufWriter::new(writable_buff);
    let encoder = PngEncoder::new_with_quality(buff, compression, filter);
    Ok(encoder.encode(&png, png.width(), png.height(), ColorType::Rgba8)?)
}

/// Writes a PNG to the local file system using the provided compression
/// and filter definitions.
///
/// # Arguments
///
/// * `file_out` - Local file system path where to write the PNG file.
/// * `png` - A byte buffer with the image data.
/// * `compression` - Compression type to use in the encoding.
/// * `filter` - Filter type to use in the encoding.
pub fn write_png_to_file(
    file_out: String,
    png: &ImageBuffer<Rgba<u8>, Vec<u8>>,
    compression: CompressionType,
    filter: FilterType,
) -> Result<(), PConvertError> {
    let file = File::create(&file_out)?;
    encode_png(file, png, compression, filter)
}

/// Writes a PNG to the local file system using the default
/// compression and filter settings.
///
/// Avoid the usage of external enumeration values.
///
/// # Arguments
///
/// * `file_out` - Local file system path where to write the PNG file.
/// * `png` - A byte buffer with the image data.
pub fn write_png_to_file_d(
    file_out: String,
    png: &ImageBuffer<Rgba<u8>, Vec<u8>>,
) -> Result<(), PConvertError> {
    let file = File::create(&file_out)?;
    encode_png(file, png, CompressionType::Fast, FilterType::NoFilter)
}

/// [NOT SUPPORTED IN WASM] Multi-threaded write version of a
/// PNG to the local file system.
///
/// # Arguments
///
/// * `file_out` - Local file system path where to write the PNG file.
/// * `png` - A byte buffer with the image data.
/// * `compression` - Compression type to use in the encoding.
/// * `filter` - Filter type to use in the encoding.
#[cfg(not(feature = "wasm-extension"))]
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

/// [SUPPORTED IN WASM] WASM stub; single-threaded write PNG to the
/// local file system.
///
/// # Arguments
///
/// * `file_out` - Local file system path where to write the PNG file.
/// * `png` - A byte buffer with the image data.
/// * `compression` - Compression type to use in the encoding.
/// * `filter` - Filter type to use in the encoding.
#[cfg(feature = "wasm-extension")]
pub fn write_png_parallel(
    file_out: String,
    png: &ImageBuffer<Rgba<u8>, Vec<u8>>,
    compression: CompressionType,
    filter: FilterType,
) -> Result<(), PConvertError> {
    write_png_to_file(file_out, png, compression, filter)
}

/// Converts a `String` to a `image::codecs::png::CompressionType`.
/// This can not be done by implementing the trait `From<String> for CompressionType` due to Rust's.
/// [orphan rule](https://doc.rust-lang.org/book/ch10-02-traits.html#implementing-a-trait-on-a-type).
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

/// Converts a `String` to a `image::codecs::png::FilterType`.
/// This can not be done by implementing the trait `From<String> for FilterType` due to Rust's
/// [orphan rule](https://doc.rust-lang.org/book/ch10-02-traits.html#implementing-a-trait-on-a-type).
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

#[cfg(not(feature = "wasm-extension"))]
fn mtpng_compression_from(compression: CompressionType) -> mtpng::CompressionLevel {
    match compression {
        CompressionType::Default => mtpng::CompressionLevel::Default,
        CompressionType::Best => mtpng::CompressionLevel::High,
        CompressionType::Fast => mtpng::CompressionLevel::Fast,
        _ => mtpng::CompressionLevel::Fast,
    }
}

#[cfg(not(feature = "wasm-extension"))]
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

/// Maximum of two values that implement the `PartialOrd` trait.
pub fn max<T: PartialOrd>(x: T, y: T) -> T {
    if x > y {
        x
    } else {
        y
    }
}

/// Minimum of two values that implement the `PartialOrd` trait.
pub fn min<T: PartialOrd>(x: T, y: T) -> T {
    if x < y {
        x
    } else {
        y
    }
}
