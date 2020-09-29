#[macro_use]
mod utils;

mod benchmark;
mod conversions;

use crate::blending::params::BlendAlgorithmParams;
use crate::blending::{
    blend_images, demultiply_image, get_blending_algorithm, is_algorithm_multiplied, BlendAlgorithm,
};
use crate::constants;
use crate::errors::PConvertError;
use image::{ImageBuffer, Rgba, RgbaImage};
use js_sys::try_iter;
use serde_json::json;
use utils::{build_algorithm, build_params, encode_file, encode_image_data, load_png};
use wasm_bindgen::prelude::*;
use web_sys::{File, ImageData};

#[wasm_bindgen(js_name = blendImages)]
pub async fn blend_images_js(
    top: File,
    bot: File,
    target_file_name: String,
    algorithm: Option<String>,
    is_inline: Option<bool>,
) -> Result<File, JsValue> {
    let mut top = load_png(top, false).await?;
    let mut bot = load_png(bot, false).await?;

    blend_image_buffers(&mut top, &mut bot, algorithm, is_inline)?;

    encode_file(bot)
}

#[wasm_bindgen(js_name = blendImagesData)]
pub fn blend_images_data_js(
    top: ImageData,
    bot: ImageData,
    algorithm: Option<String>,
    is_inline: Option<bool>,
) -> Result<ImageData, JsValue> {
    let (width, height) = (top.width(), top.height());
    let mut top = ImageBuffer::from_vec(width, height, top.data().to_vec()).ok_or(
        PConvertError::ArgumentError("Could not parse \"top\"".to_string()),
    )?;
    let mut bot = ImageBuffer::from_vec(width, height, bot.data().to_vec()).ok_or(
        PConvertError::ArgumentError("Could not parse \"bot\"".to_string()),
    )?;

    blend_image_buffers(&mut top, &mut bot, algorithm, is_inline)?;

    encode_image_data(bot)
}

pub fn blend_image_buffers(
    top: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
    bot: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
    algorithm: Option<String>,
    is_inline: Option<bool>,
) -> Result<(), PConvertError> {
    let algorithm = algorithm.unwrap_or(String::from("multiplicative"));
    let algorithm = build_algorithm(&algorithm)?;
    let algorithm_fn = get_blending_algorithm(&algorithm);
    let demultiply = is_algorithm_multiplied(&algorithm);
    let _is_inline = is_inline.unwrap_or(false);

    if demultiply {
        demultiply_image(top);
        demultiply_image(bot);
    }

    blend_images(&top, bot, &algorithm_fn, &None);
    Ok(())
}

#[wasm_bindgen(js_name = blendMultiple)]
pub async fn blend_multiple_js(
    image_files: JsValue,
    target_file_name: String,
    algorithm: Option<String>,
    algorithms: Option<Box<[JsValue]>>,
    is_inline: Option<bool>,
) -> Result<File, JsValue> {
    let mut image_buffers = Vec::new();
    let image_files = try_iter(&image_files).unwrap().unwrap();
    for file in image_files {
        let file = file?;
        let img = load_png(file.into(), false).await?;

        image_buffers.push(img);
    }

    let composition = blend_multiple_buffers(image_buffers, algorithm, algorithms, is_inline)?;

    encode_file(composition)
}

#[wasm_bindgen(js_name = blendMultipleData)]
pub fn blend_multiple_data_js(
    images: &JsValue,
    algorithm: Option<String>,
    algorithms: Option<Box<[JsValue]>>,
    is_inline: Option<bool>,
) -> Result<ImageData, JsValue> {
    let mut image_buffers: Vec<RgbaImage> = Vec::new();
    let mut images = try_iter(images).unwrap().unwrap();
    while let Some(Ok(img_data)) = images.next() {
        let img_data: ImageData = img_data.into();
        let img_buffer: RgbaImage = ImageBuffer::from_vec(
            img_data.width(),
            img_data.height(),
            img_data.data().to_vec(),
        )
        .ok_or(PConvertError::ArgumentError(
            "Could not parse \"bot\"".to_string(),
        ))?;

        image_buffers.push(img_buffer);
    }

    let composition = blend_multiple_buffers(image_buffers, algorithm, algorithms, is_inline)?;

    encode_image_data(composition)
}

fn blend_multiple_buffers(
    image_buffers: Vec<ImageBuffer<Rgba<u8>, Vec<u8>>>,
    algorithm: Option<String>,
    algorithms: Option<Box<[JsValue]>>,
    is_inline: Option<bool>,
) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>, PConvertError> {
    let num_images = image_buffers.len();
    if num_images < 1 {
        return Err(PConvertError::ArgumentError(
            "'images' must contain at least one path".to_string(),
        ));
    }

    if algorithms.is_some() && algorithms.as_ref().unwrap().len() != num_images - 1 {
        return Err(PConvertError::ArgumentError(format!(
            "'algorithms' must be of size {} (one per blending operation)",
            num_images - 1
        )));
    };

    let _is_inline = is_inline.unwrap_or(false);

    let algorithms_to_apply: Vec<(BlendAlgorithm, Option<BlendAlgorithmParams>)> =
        if let Some(algorithms) = algorithms {
            build_params(algorithms)?
        } else if let Some(algorithm) = algorithm {
            let algorithm = build_algorithm(&algorithm)?;
            vec![(algorithm, None); num_images - 1]
        } else {
            vec![(BlendAlgorithm::Multiplicative, None); num_images - 1]
        };

    let mut image_buffers_iter = image_buffers.iter();
    let first_demultiply = is_algorithm_multiplied(&algorithms_to_apply[0].0);
    let mut composition = image_buffers_iter.next().unwrap().to_owned();
    if first_demultiply {
        demultiply_image(&mut composition);
    }
    let mut zip_iter = image_buffers_iter.zip(algorithms_to_apply.iter());
    while let Some(pair) = zip_iter.next() {
        let mut current_layer = pair.0.to_owned();
        let (algorithm, algorithm_params) = pair.1;
        let demultiply = is_algorithm_multiplied(&algorithm);
        let algorithm_fn = get_blending_algorithm(&algorithm);

        if demultiply {
            demultiply_image(&mut current_layer);
        }

        blend_images(
            &current_layer,
            &mut composition,
            &algorithm_fn,
            algorithm_params,
        );
    }

    Ok(composition)
}

#[wasm_bindgen(js_name = getModuleConstants)]
pub fn get_module_constants_js() -> JsValue {
    JsValue::from_serde(&json!({
        "COMPILATION_DATE": constants::COMPILATION_DATE,
        "COMPILATION_TIME": constants::COMPILATION_TIME,
        "VERSION": constants::VERSION,
        "ALGORITHMS": constants::ALGORITHMS,
        "COMPILER": constants::COMPILER,
        "COMPILER_VERSION": constants::COMPILER_VERSION,
        "LIBPNG_VERSION": constants::LIBPNG_VERSION,
        "FEATURES": constants::FEATURES,
        "PLATFORM_CPU_BITS": constants::PLATFORM_CPU_BITS
    }))
    .unwrap()
}
