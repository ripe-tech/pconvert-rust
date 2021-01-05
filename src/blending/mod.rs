//! Blending algorithms and associated utility functions and enums.

pub mod algorithms;
pub mod params;

use algorithms::{
    blend_alpha, blend_destination_over, blend_disjoint_debug, blend_disjoint_over,
    blend_disjoint_under, blend_first_bottom, blend_first_top, blend_mask_top,
    blend_multiplicative, blend_source_over,
};
use image::{ImageBuffer, Rgba};
use params::BlendAlgorithmParams;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::result;
use std::str::FromStr;

/// Enumeration of supported blending modes.
#[derive(Clone, Debug)]
pub enum BlendAlgorithm {
    Alpha,
    Multiplicative,
    SourceOver,
    DestinationOver,
    MaskTop,
    FirstTop,
    FirstBottom,
    DisjointOver,
    DisjointUnder,
    DisjointDebug,
}

impl FromStr for BlendAlgorithm {
    type Err = String;

    fn from_str(s: &str) -> result::Result<Self, Self::Err> {
        match s {
            "alpha" => Ok(BlendAlgorithm::Alpha),
            "multiplicative" => Ok(BlendAlgorithm::Multiplicative),
            "source_over" => Ok(BlendAlgorithm::SourceOver),
            "destination_over" => Ok(BlendAlgorithm::DestinationOver),
            "mask_top" => Ok(BlendAlgorithm::MaskTop),
            "first_top" => Ok(BlendAlgorithm::FirstTop),
            "first_bottom" => Ok(BlendAlgorithm::FirstBottom),
            "disjoint_over" => Ok(BlendAlgorithm::DisjointOver),
            "disjoint_under" => Ok(BlendAlgorithm::DisjointUnder),
            "disjoint_debug" => Ok(BlendAlgorithm::DisjointDebug),
            s => Err(s.to_string()),
        }
    }
}

impl Display for BlendAlgorithm {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            BlendAlgorithm::Alpha => write!(f, "alpha"),
            BlendAlgorithm::Multiplicative => write!(f, "multiplicative"),
            BlendAlgorithm::SourceOver => write!(f, "source_over"),
            BlendAlgorithm::DestinationOver => write!(f, "destination_over"),
            BlendAlgorithm::MaskTop => write!(f, "mask_top"),
            BlendAlgorithm::FirstTop => write!(f, "first_top"),
            BlendAlgorithm::FirstBottom => write!(f, "first_bottom"),
            BlendAlgorithm::DisjointOver => write!(f, "disjoint_over"),
            BlendAlgorithm::DisjointUnder => write!(f, "disjoint_under"),
            BlendAlgorithm::DisjointDebug => write!(f, "disjoint_debug"),
        }
    }
}

/// Blends two images buffers with the given blending function and
/// optional parameters.
///
/// # Arguments
///
/// * `bot` - An image buffer corresponding to the bottom layer, in typical
/// composition language this should be considered the `source`.
/// * `top` - An image buffer corresponding to the top layer, in typical
/// composition language this should be considered the `destination`.
/// * `blending_algorithm` - A function that blends two pixels according
/// to optional blending parameters.
/// * `algorithm_params` - A optional map of key-value pairs of blending
/// properties and values.
///
/// # Examples
///
/// ```no_run
/// use pconvert_rust::blending::{blend_images, get_blending_algorithm, BlendAlgorithm};
/// use pconvert_rust::utils::read_png_from_file;
///
/// let mut bot = read_png_from_file("bot.png".to_string(), false).unwrap();
/// let top = read_png_from_file("top.png".to_string(), false).unwrap();
/// let algorithm_fn = get_blending_algorithm(&BlendAlgorithm::Alpha);
///
/// blend_images(&mut bot, &top, &algorithm_fn, &None);
/// ```
pub fn blend_images(
    bot: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
    top: &ImageBuffer<Rgba<u8>, Vec<u8>>,
    blending_algorithm: &impl Fn((&mut Rgba<u8>, &Rgba<u8>), &Option<BlendAlgorithmParams>),
    algorithm_params: &Option<BlendAlgorithmParams>,
) {
    for pixel_pair in bot.pixels_mut().zip(top.pixels()) {
        blending_algorithm(pixel_pair, algorithm_params);
    }
}

/// Demultiplies an image buffer, by applying the demultiply operation over the
/// complete set of pixels in the provided image buffer.
///
/// # Arguments
///
/// * `img` - The image buffer to demultiply.
pub fn demultiply_image(img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>) {
    for pixel in img.pixels_mut() {
        demultiply_pixel(pixel);
    }
}

/// Multiplies an image buffer, running the opposite operation over the
/// complete set of pixels in the image buffer.
///
/// # Arguments
///
/// * `img` - The image buffer to multiply.
pub fn multiply_image(img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>) {
    for pixel in img.pixels_mut() {
        multiply_pixel(pixel);
    }
}

/// Matches a `BlendAlgorithm` enum variant with a blend function.
///
/// # Arguments
///
/// * `algorithm` - The BlendAlgorithm enum variant.
pub fn get_blending_algorithm(
    algorithm: &BlendAlgorithm,
) -> impl Fn((&mut Rgba<u8>, &Rgba<u8>), &Option<BlendAlgorithmParams>) {
    match algorithm {
        BlendAlgorithm::Alpha => blend_alpha,
        BlendAlgorithm::Multiplicative => blend_multiplicative,
        BlendAlgorithm::SourceOver => blend_source_over,
        BlendAlgorithm::DestinationOver => blend_destination_over,
        BlendAlgorithm::MaskTop => blend_mask_top,
        BlendAlgorithm::FirstTop => blend_first_top,
        BlendAlgorithm::FirstBottom => blend_first_bottom,
        BlendAlgorithm::DisjointOver => blend_disjoint_over,
        BlendAlgorithm::DisjointUnder => blend_disjoint_under,
        BlendAlgorithm::DisjointDebug => blend_disjoint_debug,
    }
}

/// Returns whether or not a `BlendAlgorithm` enum variant corresponds to a
/// multiplied blending algorithm.
///
/// # Arguments
///
/// * `algorithm` - The BlendAlgorithm enum variant.
pub fn is_algorithm_multiplied(algorithm: &BlendAlgorithm) -> bool {
    match algorithm {
        BlendAlgorithm::Alpha => false,
        BlendAlgorithm::Multiplicative => false,
        BlendAlgorithm::SourceOver => false,
        BlendAlgorithm::DestinationOver => false,
        BlendAlgorithm::MaskTop => false,
        BlendAlgorithm::FirstTop => false,
        BlendAlgorithm::FirstBottom => false,
        BlendAlgorithm::DisjointOver => true,
        BlendAlgorithm::DisjointUnder => true,
        BlendAlgorithm::DisjointDebug => true,
    }
}

fn demultiply_pixel(pixel: &mut Rgba<u8>) {
    let (r, g, b, a) = (pixel[0], pixel[1], pixel[2], pixel[3]);
    let af = a as f32 / 255.0;

    let r = (r as f32 * af).round() as u8;
    let g = (g as f32 * af).round() as u8;
    let b = (b as f32 * af).round() as u8;

    pixel[0] = r;
    pixel[1] = g;
    pixel[2] = b;
}

fn multiply_pixel(pixel: &mut Rgba<u8>) {
    let (r, g, b, a) = (pixel[0], pixel[1], pixel[2], pixel[3]);
    let af = a as f32 / 255.0;

    let r = (r as f32 / af).round() as u8;
    let g = (g as f32 / af).round() as u8;
    let b = (b as f32 / af).round() as u8;

    pixel[0] = r;
    pixel[1] = g;
    pixel[2] = b;
}
