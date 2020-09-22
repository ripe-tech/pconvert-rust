use super::blending;
use super::blending::{get_blending_algorithm, BlendAlgorithm};
use crate::errors::PConvertError;
use image::{ImageBuffer, RgbaImage};
use js_sys::{Function, Promise};
use std::str::FromStr;
use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{File, HtmlCanvasElement, HtmlImageElement, ImageData, Url};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub async fn blend_images(top: File, bot: File) -> Result<ImageData, JsValue> {
    let top = JsFuture::from(load_image(top)).await?;
    let bot = JsFuture::from(load_image(bot)).await?;

    let top = get_image_data(top.into())?;
    let bot = get_image_data(bot.into())?;

    blend_images_data(top, bot, None, None)
}

#[wasm_bindgen]
pub fn blend_images_data(
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

fn load_image(file: File) -> Promise {
    Promise::new(&mut |resolve, reject| {
        let img = HtmlImageElement::new().unwrap();
        let on_load = Function::new_with_args("resolve, img, e", "resolve(img)");
        let on_load = on_load.bind2(&JsValue::NULL, &resolve, &img);
        img.set_onload(Some(&on_load));

        let on_err = Function::new_with_args("reject", "reject(\"Failed loading image URL\")");
        let on_err = on_err.bind1(&JsValue::NULL, &reject);
        img.set_onerror(Some(&on_err));

        let url = Url::create_object_url_with_blob(&file).unwrap();
        img.set_src(&url);
    })
}

fn get_image_data(img: HtmlImageElement) -> Result<ImageData, JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.create_element("canvas").unwrap();
    let canvas: HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();
    canvas.set_width(img.width());
    canvas.set_height(img.height());
    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();
    context.draw_image_with_html_image_element(&img, 0.0, 0.0)?;
    context.get_image_data(0.0, 0.0, img.width().into(), img.height().into())
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
