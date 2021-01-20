//! Benchmarking functions of the WASM API exposed

use crate::constants;
use crate::wasm::utils::{encode_file, load_png, log_benchmark, log_benchmark_header};
use crate::wasm::{blend_image_buffers, blend_multiple_buffers};
use image::codecs::png::{CompressionType, FilterType};
use js_sys::try_iter;
use wasm_bindgen::prelude::*;
use web_sys::File;

/// Benchmarks the `blend_images_js` API method for all combinations of
/// algorithms, compression and filter types.
#[wasm_bindgen(js_name = blendImagesBenchmarkAll)]
pub async fn blend_images_benchmark_all_js(
    bot: File,
    top: File,
    is_inline: Option<bool>,
) -> Result<(), JsValue> {
    log_benchmark_header();
    for algorithm in constants::ALGORITHMS.iter() {
        for compression in constants::COMPRESSION_TYPES.iter() {
            for filter in constants::FILTER_TYPES.iter() {
                blend_images_benchmark_js(
                    bot.clone(),
                    top.clone(),
                    "".to_string(),
                    Some(algorithm.to_string()),
                    is_inline,
                    *compression,
                    *filter,
                )
                .await?;
            }
        }
    }
    Ok(())
}

/// Benchmarks the `blend_multiple_js` API method for all combinations of
/// algorithms, compression and filter types.
#[wasm_bindgen(js_name = blendMultipleBenchmarkAll)]
pub async fn blend_multiple_benchmark_all_js(
    image_files: JsValue,
    is_inline: Option<bool>,
) -> Result<(), JsValue> {
    log_benchmark_header();
    for algorithm in constants::ALGORITHMS.iter() {
        for compression in constants::COMPRESSION_TYPES.iter() {
            for filter in constants::FILTER_TYPES.iter() {
                blend_multiple_benchmark_js(
                    image_files.clone(),
                    "".to_string(),
                    Some(algorithm.to_string()),
                    None,
                    is_inline,
                    *compression,
                    *filter,
                )
                .await?;
            }
        }
    }
    Ok(())
}

async fn blend_images_benchmark_js(
    bot: File,
    top: File,
    target_file_name: String,
    algorithm: Option<String>,
    is_inline: Option<bool>,
    compression: CompressionType,
    filter: FilterType,
) -> Result<File, JsValue> {
    let start_read = js_sys::Date::now();

    let mut bot = load_png(bot, false).await?;
    let mut top = load_png(top, false).await?;

    let start_blend = js_sys::Date::now();

    blend_image_buffers(&mut bot, &mut top, algorithm.clone(), is_inline)?;

    let start_write = js_sys::Date::now();

    let file = encode_file(bot, compression, filter, target_file_name)?;

    let end = js_sys::Date::now();

    let read_time = start_blend - start_read;
    let blend_time = start_write - start_blend;
    let write_time = end - start_write;

    log_benchmark(
        algorithm.unwrap_or_else(|| "multiplicative".to_string()),
        compression,
        filter,
        blend_time,
        read_time,
        write_time,
    );

    Ok(file)
}

async fn blend_multiple_benchmark_js(
    image_files: JsValue,
    target_file_name: String,
    algorithm: Option<String>,
    algorithms: Option<Vec<JsValue>>,
    is_inline: Option<bool>,
    compression: CompressionType,
    filter: FilterType,
) -> Result<File, JsValue> {
    let start_read = js_sys::Date::now();

    let mut image_buffers = Vec::new();
    let image_files = try_iter(&image_files).unwrap().unwrap();
    for file in image_files {
        let file = file?;
        let img = load_png(file.into(), false).await?;

        image_buffers.push(img);
    }

    let start_blend = js_sys::Date::now();

    let composition =
        blend_multiple_buffers(image_buffers, algorithm.clone(), algorithms, is_inline)?;

    let start_write = js_sys::Date::now();

    let file = encode_file(composition, compression, filter, target_file_name)?;

    let end = js_sys::Date::now();

    let read_time = start_blend - start_read;
    let blend_time = start_write - start_blend;
    let write_time = end - start_write;

    log_benchmark(
        algorithm.unwrap_or_else(|| "multiplicative".to_string()),
        compression,
        filter,
        blend_time,
        read_time,
        write_time,
    );

    Ok(file)
}
