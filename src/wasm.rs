use js_sys::try_iter;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub fn blend_images(
    bot_path: String,
    top_path: String,
    target_path: String,
    algorithm: Option<String>,
    is_inline: Option<bool>,
) {
    let debug = format!(
        "[blend_images]\n{} {} {} {:?} {:?}",
        bot_path, top_path, target_path, algorithm, is_inline
    );
    console_log!("{}", debug);
}

#[wasm_bindgen]
pub fn blend_multiple(
    img_paths: &JsValue,
    out_path: String,
    algorithm: Option<String>,
    algorithms: Option<Box<[JsValue]>>,
    is_inline: Option<bool>,
) -> Result<(), JsValue> {
    console_log!("[blend_multiple]");

    console_log!("{:?}", img_paths);
    // let img_paths = try_iter(img_paths).unwrap().unwrap();

    console_log!("{}", out_path);

    console_log!("{:?}", algorithm);

    console_log!("{:?}", algorithms);

    console_log!("{:?}", is_inline);

    Ok(())
}
