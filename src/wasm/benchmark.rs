use crate::constants::ALGORITHMS;
use crate::wasm::utils::{encode_file, load_png, log_benchmark};
use crate::wasm::{blend_image_buffers, blend_multiple_buffers};
use js_sys::try_iter;
use wasm_bindgen::prelude::*;
use web_sys::File;

#[wasm_bindgen]
pub async fn blend_images_benchmark_all(
    top: File,
    bot: File,
    is_inline: Option<bool>,
) -> Result<(), JsValue> {
    for algorithm in ALGORITHMS.iter() {
        blend_images_benchmark(
            top.clone(),
            bot.clone(),
            Some(algorithm.to_string()),
            is_inline,
        )
        .await?;
    }
    Ok(())
}

#[wasm_bindgen]
pub async fn blend_multiple_benchmark_all(
    image_files: JsValue,
    is_inline: Option<bool>,
) -> Result<(), JsValue> {
    for algorithm in ALGORITHMS.iter() {
        blend_multiple_benchmark(
            image_files.clone(),
            Some(algorithm.to_string()),
            None,
            is_inline,
        )
        .await?;
    }
    Ok(())
}

#[wasm_bindgen]
pub async fn blend_images_benchmark(
    top: File,
    bot: File,
    algorithm: Option<String>,
    is_inline: Option<bool>,
) -> Result<File, JsValue> {
    let start_read = js_sys::Date::now();

    let mut top = load_png(top, false).await?;
    let mut bot = load_png(bot, false).await?;

    let start_blend = js_sys::Date::now();

    blend_image_buffers(&mut top, &mut bot, algorithm.clone(), is_inline)?;

    let start_write = js_sys::Date::now();

    let file = encode_file(bot)?;

    let end = js_sys::Date::now();

    let read_time = start_blend - start_read;
    let blend_time = start_write - start_blend;
    let write_time = end - start_write;

    log_benchmark(
        algorithm.unwrap_or("multiplicative".to_string()),
        blend_time,
        read_time,
        write_time,
    );

    Ok(file)
}

#[wasm_bindgen]
pub async fn blend_multiple_benchmark(
    image_files: JsValue,
    algorithm: Option<String>,
    algorithms: Option<Box<[JsValue]>>,
    is_inline: Option<bool>,
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

    let file = encode_file(composition)?;

    let end = js_sys::Date::now();

    let read_time = start_blend - start_read;
    let blend_time = start_write - start_blend;
    let write_time = end - start_write;

    log_benchmark(
        algorithm.unwrap_or("multiplicative".to_string()),
        blend_time,
        read_time,
        write_time,
    );

    Ok(file)
}
