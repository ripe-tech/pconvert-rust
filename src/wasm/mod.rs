//! Web Assembly (WASM) extension, exported functions and type conversions.

#[macro_use]
pub mod utils;

pub mod benchmark;
pub mod conversions;

use crate::blending::params::BlendAlgorithmParams;
use crate::blending::{
    blend_images, demultiply_image, get_blending_algorithm, is_algorithm_multiplied, BlendAlgorithm,
};
use crate::constants;
use crate::errors::PConvertError;
use crate::utils::{decode_png, encode_png};
use image::{ImageBuffer, Rgba, RgbaImage};
use js_sys::try_iter;
use serde_json::json;
use serde_json::Value as JSONValue;
use std::collections::HashMap;
use utils::{
    build_algorithm, build_params, encode_file, encode_image_data, get_compression_type,
    get_filter_type, load_png, node_read_file_async, node_read_file_sync, node_require,
    node_write_file_sync,
};
use wasm_bindgen::prelude::*;
use web_sys::{File, ImageData};

/// Blends two `File`s into one, named `target_file_name`, using `algorithm` and the extra
/// `options` given. Algorithm defaults to `BlendAlgorithm::Multiplicative`.
#[wasm_bindgen(js_name = blendImages)]
pub async fn blend_images_js(
    bot: File,
    top: File,
    target_file_name: String,
    algorithm: Option<String>,
    is_inline: Option<bool>,
    options: JsValue,
) -> Result<File, JsValue> {
    let options = match options.is_object() {
        true => options.into_serde::<HashMap<String, JSONValue>>().ok(),
        false => None,
    };

    let mut bot = load_png(bot, false).await?;
    let mut top = load_png(top, false).await?;

    blend_image_buffers(&mut bot, &mut top, algorithm, is_inline)?;

    encode_file(
        bot,
        get_compression_type(&options),
        get_filter_type(&options),
        target_file_name,
    )
}

/// Blends two `ImageData` objects into one using `algorithm` and the extra
/// `options` given. Algorithm defaults to `BlendAlgorithm::Multiplicative`.
#[wasm_bindgen(js_name = blendImagesData)]
pub fn blend_images_data_js(
    bot: ImageData,
    top: ImageData,
    algorithm: Option<String>,
    is_inline: Option<bool>,
    options: JsValue,
) -> Result<ImageData, JsValue> {
    let options = match options.is_object() {
        true => options.into_serde::<HashMap<String, JSONValue>>().ok(),
        false => None,
    };

    let (width, height) = (bot.width(), bot.height());
    let mut bot = ImageBuffer::from_vec(width, height, bot.data().to_vec())
        .ok_or_else(|| PConvertError::ArgumentError("Could not parse \"bot\"".to_string()))?;
    let mut top = ImageBuffer::from_vec(width, height, top.data().to_vec())
        .ok_or_else(|| PConvertError::ArgumentError("Could not parse \"top\"".to_string()))?;

    blend_image_buffers(&mut bot, &mut top, algorithm, is_inline)?;

    encode_image_data(
        bot,
        get_compression_type(&options),
        get_filter_type(&options),
    )
}

/// Blends two image buffers using `algorithm` and the extra
/// `options` given. Algorithm defaults to `BlendAlgorithm::Multiplicative`.
pub fn blend_image_buffers(
    bot: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
    top: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
    algorithm: Option<String>,
    is_inline: Option<bool>,
) -> Result<(), PConvertError> {
    let algorithm = algorithm.unwrap_or_else(|| String::from("multiplicative"));
    let algorithm = build_algorithm(&algorithm)?;
    let algorithm_fn = get_blending_algorithm(&algorithm);
    let demultiply = is_algorithm_multiplied(&algorithm);
    let _is_inline = is_inline.unwrap_or(false);

    if demultiply {
        demultiply_image(bot);
        demultiply_image(top);
    }

    blend_images(bot, &top, &algorithm_fn, &None);
    Ok(())
}

/// Blends multiple `File`s into one, named `target_file_name`, using `algorithm` and the extra
/// `options` given. Algorithm defaults to `BlendAlgorithm::Multiplicative`.
#[wasm_bindgen(js_name = blendMultiple)]
pub async fn blend_multiple_js(
    image_files: JsValue,
    target_file_name: String,
    algorithm: Option<String>,
    algorithms: Option<Vec<JsValue>>,
    is_inline: Option<bool>,
    options: JsValue,
) -> Result<File, JsValue> {
    let options = match options.is_object() {
        true => options.into_serde::<HashMap<String, JSONValue>>().ok(),
        false => None,
    };

    let mut image_buffers = Vec::new();
    let image_files = try_iter(&image_files).unwrap().unwrap();
    for file in image_files {
        let file = file?;
        let img = load_png(file.into(), false).await?;

        image_buffers.push(img);
    }

    let composition = blend_multiple_buffers(image_buffers, algorithm, algorithms, is_inline)?;
    encode_file(
        composition,
        get_compression_type(&options),
        get_filter_type(&options),
        target_file_name,
    )
}

/// Blends multiple `ImageData` objects into one using `algorithm` and the extra
/// `options` given. Algorithm defaults to `BlendAlgorithm::Multiplicative`.
#[wasm_bindgen(js_name = blendMultipleData)]
pub fn blend_multiple_data_js(
    images: &JsValue,
    algorithm: Option<String>,
    algorithms: Option<Vec<JsValue>>,
    is_inline: Option<bool>,
    options: JsValue,
) -> Result<ImageData, JsValue> {
    let options = match options.is_object() {
        true => options.into_serde::<HashMap<String, JSONValue>>().ok(),
        false => None,
    };

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

    let composition = blend_multiple_buffers(image_buffers, algorithm, algorithms, is_inline)?;
    encode_image_data(
        composition,
        get_compression_type(&options),
        get_filter_type(&options),
    )
}

/// Returns a JSON object with the module constants (e.g. ALGORITHMS, COMPILER, COMPILER_VERSION, ...).
#[wasm_bindgen(js_name = getModuleConstants)]
pub fn get_module_constants_js() -> JsValue {
    let filters: Vec<String> = constants::FILTER_TYPES
        .to_vec()
        .iter()
        .map(|x| format!("{:?}", x))
        .collect();

    let compressions: Vec<String> = constants::COMPRESSION_TYPES
        .to_vec()
        .iter()
        .map(|x| format!("{:?}", x))
        .collect();

    JsValue::from_serde(&json!({
        "COMPILATION_DATE": constants::COMPILATION_DATE,
        "COMPILATION_TIME": constants::COMPILATION_TIME,
        "VERSION": constants::VERSION,
        "ALGORITHMS": constants::ALGORITHMS,
        "COMPILER": constants::COMPILER,
        "COMPILER_VERSION": constants::COMPILER_VERSION,
        "LIBPNG_VERSION": constants::LIBPNG_VERSION,
        "FEATURES": constants::FEATURES,
        "PLATFORM_CPU_BITS": constants::PLATFORM_CPU_BITS,
        "FILTER_TYPES": filters,
        "COMPRESSION_TYPES": compressions
    }))
    .unwrap()
}

/// [NodeJS only]
/// Blends multiple images read from local file system into one using `algorithm` or `algorithms` and the extra
/// `options` given. Algorithm defaults to `BlendAlgorithm::Multiplicative`.
#[wasm_bindgen(js_name = blendMultipleFs)]
pub fn blend_multiple_fs(
    image_paths: Vec<JsValue>,
    out_path: String,
    algorithm: Option<String>,
    algorithms: Option<Vec<JsValue>>,
    is_inline: Option<bool>,
    options: JsValue,
) -> Result<(), JsValue> {
    let num_images = image_paths.len();

    if num_images < 1 {
        return Err(PConvertError::ArgumentError(
            "ArgumentError: 'img_paths' must contain at least one path".to_string(),
        )
        .into());
    }

    let options = match options.is_object() {
        true => options.into_serde::<HashMap<String, JSONValue>>().ok(),
        false => None,
    };

    let algorithms_to_apply: Vec<(BlendAlgorithm, Option<BlendAlgorithmParams>)> =
        if let Some(algorithms) = algorithms {
            build_params(&algorithms)?
        } else if let Some(algorithm) = algorithm {
            let algorithm = build_algorithm(&algorithm)?;
            vec![(algorithm, None); num_images - 1]
        } else {
            vec![(BlendAlgorithm::Multiplicative, None); num_images - 1]
        };

    if algorithms_to_apply.len() != num_images - 1 {
        return Err(PConvertError::ArgumentError(format!(
            "ArgumentError: 'algorithms' must be of size {} (one per blending operation)",
            num_images - 1
        ))
        .into());
    };

    let _is_inline = is_inline.unwrap_or(false);

    // loops through the algorithms to apply and blends the
    // current composition with the next layer
    let mut img_paths_iter = image_paths.iter();
    let first_path = img_paths_iter
        .next()
        .unwrap()
        .as_string()
        .expect("path must be a string");

    let node_fs = node_require("fs");

    let first_demultiply = if algorithms_to_apply.len() > 0 {
        is_algorithm_multiplied(&algorithms_to_apply[0].0)
    } else {
        false
    };
    let composition = node_read_file_sync(&node_fs, &first_path);
    let mut composition = decode_png(&composition[..], first_demultiply)?;

    let zip_iter = img_paths_iter.zip(algorithms_to_apply.iter());
    for pair in zip_iter {
        let path = pair.0.as_string().expect("path must be a string");
        let (algorithm, algorithm_params) = pair.1;
        let demultiply = is_algorithm_multiplied(&algorithm);
        let algorithm_fn = get_blending_algorithm(&algorithm);
        let current_layer = node_read_file_sync(&node_fs, &path);
        let current_layer = decode_png(&current_layer[..], demultiply)?;
        blend_images(
            &mut composition,
            &current_layer,
            &algorithm_fn,
            algorithm_params,
        );
    }

    let compression_type = get_compression_type(&options);
    let filter_type = get_filter_type(&options);

    let mut encoded_data = Vec::<u8>::with_capacity(composition.to_vec().capacity());
    encode_png(
        &mut encoded_data,
        &composition,
        compression_type,
        filter_type,
    )?;

    node_write_file_sync(&node_fs, &out_path, &encoded_data);

    Ok(())
}

/// [NodeJS only]
/// Asynchronously blends multiple images read from local file system into one using `algorithm` or `algorithms` and the extra
/// `options` given. Algorithm defaults to `BlendAlgorithm::Multiplicative`.
#[wasm_bindgen(js_name = blendMultipleFsAsync)]
pub async fn blend_multiple_fs_async(
    image_paths: Vec<JsValue>,
    out_path: String,
    algorithm: Option<String>,
    algorithms: Option<Vec<JsValue>>,
    is_inline: Option<bool>,
    options: JsValue,
) -> Result<(), JsValue> {
    let num_images = image_paths.len();

    if num_images < 1 {
        return Err(PConvertError::ArgumentError(
            "ArgumentError: 'img_paths' must contain at least one path".to_string(),
        )
        .into());
    }

    let options = match options.is_object() {
        true => options.into_serde::<HashMap<String, JSONValue>>().ok(),
        false => None,
    };

    let algorithms_to_apply: Vec<(BlendAlgorithm, Option<BlendAlgorithmParams>)> =
        if let Some(algorithms) = algorithms {
            build_params(&algorithms)?
        } else if let Some(algorithm) = algorithm {
            let algorithm = build_algorithm(&algorithm)?;
            vec![(algorithm, None); num_images - 1]
        } else {
            vec![(BlendAlgorithm::Multiplicative, None); num_images - 1]
        };

    if algorithms_to_apply.len() != num_images - 1 {
        return Err(PConvertError::ArgumentError(format!(
            "ArgumentError: 'algorithms' must be of size {} (one per blending operation)",
            num_images - 1
        ))
        .into());
    };

    let _is_inline = is_inline.unwrap_or(false);

    let node_fs = node_require("fs");

    let mut png_futures: Vec<Option<wasm_bindgen_futures::JsFuture>> =
        Vec::with_capacity(num_images);
    for path in image_paths.iter() {
        let path = path.as_string().expect("path must be a string");
        let png_future = node_read_file_async(&node_fs, &path);
        png_futures.push(Some(png_future));
    }

    let first_demultiply = if algorithms_to_apply.len() > 0 {
        is_algorithm_multiplied(&algorithms_to_apply[0].0)
    } else {
        false
    };
    let composition = png_futures[0].take().unwrap().await?;
    let composition = js_sys::Uint8Array::from(composition).to_vec();
    let mut composition = decode_png(&composition[..], first_demultiply)?;

    // loops through the algorithms to apply and blends the
    // current composition with the next layer
    // retrieves the images from the result channels
    for i in 1..png_futures.len() {
        let (algorithm, algorithm_params) = &algorithms_to_apply[i - 1];
        let demultiply = is_algorithm_multiplied(&algorithm);
        let algorithm_fn = get_blending_algorithm(&algorithm);
        let current_layer = png_futures[i].take().unwrap().await?;
        let current_layer = js_sys::Uint8Array::from(current_layer).to_vec();
        let current_layer = decode_png(&current_layer[..], demultiply)?;

        blend_images(
            &mut composition,
            &current_layer,
            &algorithm_fn,
            algorithm_params,
        );
    }

    let compression_type = get_compression_type(&options);
    let filter_type = get_filter_type(&options);

    let mut encoded_data = Vec::<u8>::with_capacity(composition.to_vec().capacity());
    encode_png(
        &mut encoded_data,
        &composition,
        compression_type,
        filter_type,
    )?;

    node_write_file_sync(&node_fs, &out_path, &encoded_data);

    Ok(())
}

fn blend_multiple_buffers(
    image_buffers: Vec<ImageBuffer<Rgba<u8>, Vec<u8>>>,
    algorithm: Option<String>,
    algorithms: Option<Vec<JsValue>>,
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
            build_params(&algorithms)?
        } else if let Some(algorithm) = algorithm {
            let algorithm = build_algorithm(&algorithm)?;
            vec![(algorithm, None); num_images - 1]
        } else {
            vec![(BlendAlgorithm::Multiplicative, None); num_images - 1]
        };

    let mut image_buffers_iter = image_buffers.iter();
    let first_demultiply = if algorithms_to_apply.len() > 0 {
        is_algorithm_multiplied(&algorithms_to_apply[0].0)
    } else {
        false
    };
    let mut composition = image_buffers_iter.next().unwrap().to_owned();
    if first_demultiply {
        demultiply_image(&mut composition);
    }
    let zip_iter = image_buffers_iter.zip(algorithms_to_apply.iter());
    for pair in zip_iter {
        let mut current_layer = pair.0.to_owned();
        let (algorithm, algorithm_params) = pair.1;
        let demultiply = is_algorithm_multiplied(&algorithm);
        let algorithm_fn = get_blending_algorithm(&algorithm);

        if demultiply {
            demultiply_image(&mut current_layer);
        }

        blend_images(
            &mut composition,
            &current_layer,
            &algorithm_fn,
            algorithm_params,
        );
    }

    Ok(composition)
}
