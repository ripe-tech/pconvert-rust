mod benchmark;
mod conversions;
mod utils;

use crate::blending;
use crate::blending::params::BlendAlgorithmParams;
use crate::blending::{
    demultiply_image, get_blending_algorithm, is_algorithm_multiplied, BlendAlgorithm,
};
use crate::constants;
use crate::errors::PConvertError;
use image::{ImageBuffer, RgbaImage};
use js_sys::{try_iter, Array};
use serde_json::json;
use utils::{build_algorithm, build_params, get_image_data, image_data_to_blob, load_image};
use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use wasm_bindgen_futures::JsFuture;
use web_sys::Blob;
use web_sys::{File, ImageData};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub async fn blend_images(
    top: File,
    bot: File,
    algorithm: Option<String>,
    is_inline: Option<bool>,
) -> Result<File, JsValue> {
    let top = JsFuture::from(load_image(top)).await?;
    let bot = JsFuture::from(load_image(bot)).await?;

    let top = get_image_data(top.into())?;
    let bot = get_image_data(bot.into())?;

    let image_data = blend_images_data(top, bot, algorithm, is_inline)?;

    let image_blob = JsFuture::from(image_data_to_blob(image_data)).await?.into();
    File::new_with_blob_sequence(&Array::of1(&image_blob), "result")
}

#[wasm_bindgen]
pub fn blend_images_data(
    top: ImageData,
    bot: ImageData,
    algorithm: Option<String>,
    is_inline: Option<bool>,
) -> Result<ImageData, JsValue> {
    let algorithm = algorithm.unwrap_or(String::from("multiplicative"));
    let algorithm = build_algorithm(&algorithm)?;

    let _is_inline = is_inline.unwrap_or(false);

    let algorithm_fn = get_blending_algorithm(&algorithm);
    let demultiply = is_algorithm_multiplied(&algorithm);

    let (width, height) = (top.width(), top.height());
    let mut top: RgbaImage = ImageBuffer::from_vec(width, height, top.data().to_vec())
        .ok_or_else(|| PConvertError::ArgumentError("Could not parse \"top\"".to_string()))?;

    let mut bot: RgbaImage = ImageBuffer::from_vec(width, height, bot.data().to_vec())
        .ok_or_else(|| PConvertError::ArgumentError("Could not parse \"bot\"".to_string()))?;

    if demultiply {
        demultiply_image(&mut top);
        demultiply_image(&mut bot);
    }

    blending::blend_images(&top, &mut bot, &algorithm_fn, &None);

    let bot_bytes = &mut bot.to_vec();
    let clamped_bot_bytes: Clamped<&mut [u8]> = Clamped(bot_bytes);
    let result = ImageData::new_with_u8_clamped_array_and_sh(clamped_bot_bytes, width, height)?;
    Ok(result)
}

#[wasm_bindgen]
pub async fn blend_multiple(
    image_files: JsValue,
    algorithm: Option<String>,
    algorithms: Option<Box<[JsValue]>>,
    is_inline: Option<bool>,
) -> Result<File, JsValue> {
    let images_data = Array::new();
    let image_files = try_iter(&image_files).unwrap().unwrap();
    for file in image_files {
        let file = file?;
        let img = JsFuture::from(load_image(file.into())).await?;

        let img = get_image_data(img.into())?;
        images_data.push(&img);
    }

    let image_data = blend_multiple_data(&images_data, algorithm, algorithms, is_inline)?;

    let image_blob = JsFuture::from(image_data_to_blob(image_data)).await?.into();
    File::new_with_blob_sequence(&Array::of1(&image_blob), "result")
}

#[wasm_bindgen]
pub fn blend_multiple_data(
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
        .ok_or_else(|| PConvertError::ArgumentError("Could not parse \"bot\"".to_string()))?;

        image_buffers.push(img_buffer);
    }

    let num_images = image_buffers.len();
    if num_images < 1 {
        return Err(JsValue::from(PConvertError::ArgumentError(
            "'images' must contain at least one path".to_string(),
        )));
    }

    if algorithms.is_some() && algorithms.as_ref().unwrap().len() != num_images - 1 {
        return Err(JsValue::from(PConvertError::ArgumentError(format!(
            "'algorithms' must be of size {} (one per blending operation)",
            num_images - 1
        ))));
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

        blending::blend_images(
            &current_layer,
            &mut composition,
            &algorithm_fn,
            algorithm_params,
        );
    }

    let composition_bytes = &mut composition.to_vec();
    let clamped_composition_bytes: Clamped<&mut [u8]> = Clamped(composition_bytes);
    let result = ImageData::new_with_u8_clamped_array_and_sh(
        clamped_composition_bytes,
        composition.width(),
        composition.height(),
    )?;
    Ok(result)
}

#[wasm_bindgen]
pub fn get_module_constants() -> JsValue {
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
