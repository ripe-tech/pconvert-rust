use crate::wasm::blend_images_data;
use crate::wasm::utils::{get_image_data, load_image};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{File, ImageData};

// #[wasm_bindgen]
// extern "C" {
//     #[wasm_bindgen(js_namespace = console)]
//     fn log(s: &str);
// }

// macro_rules! console_log {
//     ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
// }

#[wasm_bindgen]
pub async fn blend_images_benchmark(
    top: File,
    bot: File,
    algorithm: Option<String>,
    is_inline: Option<bool>,
) -> Result<ImageData, JsValue> {
    // let start_read = js_sys::Date::now();

    let top = JsFuture::from(load_image(top)).await?;
    let bot = JsFuture::from(load_image(bot)).await?;

    let top = get_image_data(top.into())?;
    let bot = get_image_data(bot.into())?;

    // let start_blend = js_sys::Date::now();

    let composition = blend_images_data(top, bot, algorithm, is_inline)?;

    // let end_blend = js_sys::Date::now();

    Ok(composition)
}
