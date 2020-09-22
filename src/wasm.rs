use super::blending;
use super::blending::{get_blending_algorithm, BlendAlgorithm};
use crate::errors::PConvertError;
use image::{ImageBuffer, RgbaImage};
use std::str::FromStr;
use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use web_sys::ImageData;

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
    top: ImageData,
    bot: ImageData,
    algorithm: Option<String>,
    is_inline: Option<bool>,
) -> Result<ImageData, JsValue> {
    let algorithm = algorithm.unwrap_or(String::from("multiplicative"));
    let algorithm = match BlendAlgorithm::from_str(&algorithm) {
        Ok(algorithm) => Ok(algorithm),
        Err(algorithm) => Err(PConvertError::ArgumentError(format!(
            "ArgumentError: invalid algorithm '{}'",
            algorithm
        ))),
    }?;

    let _is_inline = is_inline.unwrap_or(false);

    let algorithm_fn = get_blending_algorithm(&algorithm);

    let (width, height) = (top.width(), top.height());
    let top: RgbaImage = ImageBuffer::from_vec(width, height, top.data().to_vec())
        .ok_or_else(|| PConvertError::ArgumentError("Could not parse \"top\"".to_string()))?;

    let mut bot: RgbaImage = ImageBuffer::from_vec(width, height, bot.data().to_vec())
        .ok_or_else(|| PConvertError::ArgumentError("Could not parse \"bot\"".to_string()))?;

    blending::blend_images(&top, &mut bot, &algorithm_fn, &None);

    let bot_bytes = &mut bot.to_vec();
    let clamped_bot_bytes: Clamped<&mut [u8]> = Clamped(bot_bytes);
    let result = ImageData::new_with_u8_clamped_array_and_sh(clamped_bot_bytes, width, height)?;
    Ok(result)
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

impl From<PConvertError> for JsValue {
    fn from(err: PConvertError) -> JsValue {
        match err {
            PConvertError::ArgumentError(err) => JsValue::from_str(&err),
            PConvertError::ImageLibError(err) => JsValue::from_str(&err.to_string()),
            PConvertError::UnsupportedImageTypeError => JsValue::from_str(&err.to_string()),
            PConvertError::IOError(err) => JsValue::from_str(&err.to_string()),
        }
    }
}
