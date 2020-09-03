use super::utils::{max, min};

use image::{ImageBuffer, Rgba};
use std::fmt::{Display, Formatter, Result};

pub enum BlendAlgorithm {
    Alpha,
    Multiplicative,
    SourceOver,
    DestinationOver,
}

impl Display for BlendAlgorithm {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            BlendAlgorithm::Alpha => write!(f, "alpha"),
            BlendAlgorithm::Multiplicative => write!(f, "multiplicative"),
            BlendAlgorithm::SourceOver => write!(f, "source_over"),
            BlendAlgorithm::DestinationOver => write!(f, "destination_over"),
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
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Background::Alpha => write!(f, "alpha"),
            Background::White => write!(f, "white"),
            Background::Blue => write!(f, "blue"),
            Background::Texture => write!(f, "texture"),
        }
    }
}

pub fn get_blending_algorithm(
    algorithm: &BlendAlgorithm,
) -> impl Fn((&mut Rgba<u8>, &Rgba<u8>)) -> () {
    match algorithm {
        BlendAlgorithm::Alpha => blend_alpha,
        BlendAlgorithm::Multiplicative => blend_multiplicative,
        BlendAlgorithm::SourceOver => blend_source_over,
        BlendAlgorithm::DestinationOver => blend_destination_over,
    }
}

/* Blends 2 PNGs, updating the bottom reference */
pub fn blend_images(
    top: &ImageBuffer<Rgba<u8>, Vec<u8>>,
    bot: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
    blending_algorithm: &impl Fn((&mut Rgba<u8>, &Rgba<u8>)) -> (),
) {
    bot.pixels_mut()
        .zip(top.pixels())
        .for_each(blending_algorithm);
}

fn blend_alpha((bot_pixel, top_pixel): (&mut Rgba<u8>, &Rgba<u8>)) {
    let (rb, gb, bb, ab) = (bot_pixel[0], bot_pixel[1], bot_pixel[2], bot_pixel[3]);
    let (rt, gt, bt, at) = (top_pixel[0], top_pixel[1], top_pixel[2], top_pixel[3]);

    let abf = 1.0 * (ab as f32 / 255.0);
    let atf = 1.0 * (at as f32 / 255.0);
    let af = atf + abf * (1.0 - atf);

    let r = if af == 0.0 {
        0.0
    } else {
        (rb as f32 * abf + rt as f32 * atf * (1.0 - abf)) / af
    };
    let g = if af == 0.0 {
        0.0
    } else {
        (gb as f32 * abf + gt as f32 * atf * (1.0 - abf)) / af
    };
    let b = if af == 0.0 {
        0.0
    } else {
        (bb as f32 * abf + bt as f32 * atf * (1.0 - abf)) / af
    };
    let a = max(0.0, min(255.0, (abf + atf * (1.0 - abf)) * 255.0));

    let r = max(0.0, min(255.0, r));
    let g = max(0.0, min(255.0, g));
    let b = max(0.0, min(255.0, b));

    bot_pixel[0] = r as u8;
    bot_pixel[1] = g as u8;
    bot_pixel[2] = b as u8;
    bot_pixel[3] = a as u8;
}

fn blend_multiplicative((bot_pixel, top_pixel): (&mut Rgba<u8>, &Rgba<u8>)) {
    let (rb, gb, bb, ab) = (bot_pixel[0], bot_pixel[1], bot_pixel[2], bot_pixel[3]);
    let (rt, gt, bt, at) = (top_pixel[0], top_pixel[1], top_pixel[2], top_pixel[3]);

    let atf = 1.0 * (at as f32 / 255.0);

    let r = rb as f32 * (1.0 - atf) + rt as f32 * atf;
    let g = gb as f32 * (1.0 - atf) + gt as f32 * atf;
    let b = bb as f32 * (1.0 - atf) + bt as f32 * atf;
    let a = max(0, min(255, at as u16 + ab as u16));

    let r = max(0, min(255, r as u8));
    let g = max(0, min(255, g as u8));
    let b = max(0, min(255, b as u8));

    bot_pixel[0] = r;
    bot_pixel[1] = g;
    bot_pixel[2] = b;
    bot_pixel[3] = a as u8;
}

fn blend_source_over((bot_pixel, top_pixel): (&mut Rgba<u8>, &Rgba<u8>)) {
    let (rb, gb, bb, ab) = (bot_pixel[0], bot_pixel[1], bot_pixel[2], bot_pixel[3]);
    let (rt, gt, bt, at) = (top_pixel[0], top_pixel[1], top_pixel[2], top_pixel[3]);

    let abf = 1.0 * (ab as f32 / 255.0);
    let atf = 1.0 * (at as f32 / 255.0);
    let af = abf + atf * (1.0 - abf);

    let r = if af == 0.0 {
        0.0
    } else {
        (rb as f32 * abf + rt as f32 * atf * (1.0 - abf)) / af
    };
    let g = if af == 0.0 {
        0.0
    } else {
        (gb as f32 * abf + gt as f32 * atf * (1.0 - abf)) / af
    };
    let b = if af == 0.0 {
        0.0
    } else {
        (bb as f32 * abf + bt as f32 * atf * (1.0 - abf)) / af
    };
    let a = max(0.0, min(255.0, af * 255.0));

    let r = max(0.0, min(255.0, r));
    let g = max(0.0, min(255.0, g));
    let b = max(0.0, min(255.0, b));

    bot_pixel[0] = r as u8;
    bot_pixel[1] = g as u8;
    bot_pixel[2] = b as u8;
    bot_pixel[3] = a as u8;
}

fn blend_destination_over((bot_pixel, top_pixel): (&mut Rgba<u8>, &Rgba<u8>)) {
    let (rb, gb, bb, ab) = (bot_pixel[0], bot_pixel[1], bot_pixel[2], bot_pixel[3]);
    let (rt, gt, bt, at) = (top_pixel[0], top_pixel[1], top_pixel[2], top_pixel[3]);

    let abf = 1.0 * (ab as f32 / 255.0);
    let atf = 1.0 * (at as f32 / 255.0);
    let af = atf + abf * (1.0 - atf);

    let r = if af == 0.0 {
        0.0
    } else {
        (rt as f32 * atf + rb as f32 * abf * (1.0 - atf)) / af
    };
    let g = if af == 0.0 {
        0.0
    } else {
        (gt as f32 * atf + gb as f32 * abf * (1.0 - atf)) / af
    };
    let b = if af == 0.0 {
        0.0
    } else {
        (bt as f32 * atf + bb as f32 * abf * (1.0 - atf)) / af
    };
    let a = max(0.0, min(255.0, af * 255.0));

    let r = max(0.0, min(255.0, r));
    let g = max(0.0, min(255.0, g));
    let b = max(0.0, min(255.0, b));

    bot_pixel[0] = r as u8;
    bot_pixel[1] = g as u8;
    bot_pixel[2] = b as u8;
    bot_pixel[3] = a as u8;
}
