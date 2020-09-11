mod algorithms;

use algorithms::{
    blend_alpha, blend_destination_over, blend_disjoint_debug, blend_disjoint_over,
    blend_disjoint_under, blend_first_bottom, blend_first_top, blend_mask_top,
    blend_multiplicative, blend_source_over,
};
use image::{ImageBuffer, Rgba};
use std::fmt;
use std::fmt::{Display, Formatter};
use std::result;
use std::str::FromStr;

pub enum BlendAlgorithm {
    Alpha,
    Multiplicative,
    SourceOver,
    DestinationOver,
    FirstTop,
    FirstBottom,
    DisjointOver,
    DisjointUnder,
    DisjointDebug,
    MaskTop,
}

impl FromStr for BlendAlgorithm {
    type Err = String;

    fn from_str(s: &str) -> result::Result<Self, Self::Err> {
        match s {
            "alpha" => Ok(BlendAlgorithm::Alpha),
            "multiplicative" => Ok(BlendAlgorithm::Multiplicative),
            "source_over" => Ok(BlendAlgorithm::SourceOver),
            "destination_over" => Ok(BlendAlgorithm::DestinationOver),
            "first_top" => Ok(BlendAlgorithm::FirstTop),
            "first_bottom" => Ok(BlendAlgorithm::FirstBottom),
            "disjoint_over" => Ok(BlendAlgorithm::DisjointOver),
            "disjoint_under" => Ok(BlendAlgorithm::DisjointUnder),
            "disjoint_debug" => Ok(BlendAlgorithm::DisjointDebug),
            "mask_top" => Ok(BlendAlgorithm::MaskTop),
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
            BlendAlgorithm::FirstTop => write!(f, "first_top"),
            BlendAlgorithm::FirstBottom => write!(f, "first_bottom"),
            BlendAlgorithm::DisjointOver => write!(f, "disjoint_over"),
            BlendAlgorithm::DisjointUnder => write!(f, "disjoint_under"),
            BlendAlgorithm::DisjointDebug => write!(f, "disjoint_debug"),
            BlendAlgorithm::MaskTop => write!(f, "mask_top"),
        }
    }
}

pub enum Background {
    Alpha,
    White,
    Blue,
    Texture,
}

impl Display for Background {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Background::Alpha => write!(f, "alpha"),
            Background::White => write!(f, "white"),
            Background::Blue => write!(f, "blue"),
            Background::Texture => write!(f, "texture"),
        }
    }
}

pub fn blend_images(
    top: &ImageBuffer<Rgba<u8>, Vec<u8>>,
    bot: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
    blending_algorithm: &impl Fn((&mut Rgba<u8>, &Rgba<u8>)) -> (),
) {
    for pixel_pair in bot.pixels_mut().zip(top.pixels()) {
        blending_algorithm(pixel_pair);
    }
}

pub fn demultiply_image(img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>) {
    for pixel in img.pixels_mut() {
        demultiply_pixel(pixel);
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

pub fn multiply_image(img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>) {
    for pixel in img.pixels_mut() {
        multiply_pixel(pixel);
    }
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

pub fn get_blending_algorithm(
    algorithm: &BlendAlgorithm,
) -> impl Fn((&mut Rgba<u8>, &Rgba<u8>)) -> () {
    match algorithm {
        BlendAlgorithm::Alpha => blend_alpha,
        BlendAlgorithm::Multiplicative => blend_multiplicative,
        BlendAlgorithm::SourceOver => blend_source_over,
        BlendAlgorithm::DestinationOver => blend_destination_over,
        BlendAlgorithm::FirstTop => blend_first_top,
        BlendAlgorithm::FirstBottom => blend_first_bottom,
        BlendAlgorithm::DisjointOver => blend_disjoint_over,
        BlendAlgorithm::DisjointUnder => blend_disjoint_under,
        BlendAlgorithm::DisjointDebug => blend_disjoint_debug,
        BlendAlgorithm::MaskTop => blend_mask_top,
    }
}

pub fn is_algorithm_multiplied(algorithm: &BlendAlgorithm) -> bool {
    match algorithm {
        BlendAlgorithm::Alpha => false,
        BlendAlgorithm::Multiplicative => false,
        BlendAlgorithm::SourceOver => false,
        BlendAlgorithm::DestinationOver => false,
        BlendAlgorithm::FirstTop => false,
        BlendAlgorithm::FirstBottom => false,
        BlendAlgorithm::DisjointOver => true,
        BlendAlgorithm::DisjointUnder => true,
        BlendAlgorithm::DisjointDebug => true,
        BlendAlgorithm::MaskTop => false,
    }
}
