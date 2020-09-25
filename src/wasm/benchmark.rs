use crate::wasm::utils::{get_image_data, image_data_to_blob, load_image, log_benchmark};
use crate::wasm::{blend_images_data, blend_multiple_data};
use js_sys::{try_iter, Array};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::File;

#[wasm_bindgen(js_name = blendImagesBenchmark)]
pub async fn blend_images_benchmark_js(
    top: File,
    bot: File,
    algorithm: Option<String>,
    is_inline: Option<bool>,
) -> Result<File, JsValue> {
    let start_read = js_sys::Date::now();

    let top = JsFuture::from(load_image(top)).await?;
    let bot = JsFuture::from(load_image(bot)).await?;

    let top = get_image_data(top.into())?;
    let bot = get_image_data(bot.into())?;

    let start_blend = js_sys::Date::now();

    let composition_data = blend_images_data(top, bot, algorithm.clone(), is_inline)?;

    let start_write = js_sys::Date::now();

    let composition_blob = JsFuture::from(image_data_to_blob(composition_data)?)
        .await?
        .into();
    let composition_blob = File::new_with_blob_sequence(&Array::of1(&composition_blob), "result")?;

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

    Ok(composition_blob)
}

#[wasm_bindgen(js_name = blendMultipleBenchmark)]
pub async fn blend_multiple_benchmark_js(
    image_files: JsValue,
    algorithm: Option<String>,
    algorithms: Option<Box<[JsValue]>>,
    is_inline: Option<bool>,
) -> Result<File, JsValue> {
    let start_read = js_sys::Date::now();

    let images_data = Array::new();
    let image_files = try_iter(&image_files).unwrap().unwrap();
    for file in image_files {
        let file = file?;
        let img = JsFuture::from(load_image(file.into())).await?;

        let img = get_image_data(img.into())?;
        images_data.push(&img);
    }

    let start_blend = js_sys::Date::now();

    let composition_data =
        blend_multiple_data(&images_data, algorithm.clone(), algorithms, is_inline)?;

    let start_write = js_sys::Date::now();

    let composition_blob = JsFuture::from(image_data_to_blob(composition_data)?)
        .await?
        .into();

    let composition_blob = File::new_with_blob_sequence(&Array::of1(&composition_blob), "result")?;

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

    Ok(composition_blob)
}
