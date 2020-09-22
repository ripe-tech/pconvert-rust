use super::blending;
use super::blending::params::{BlendAlgorithmParams, ParamValue};
use super::blending::{
    demultiply_image, get_blending_algorithm, is_algorithm_multiplied, BlendAlgorithm,
};
use crate::errors::PConvertError;
use image::{ImageBuffer, RgbaImage};
use js_sys::{try_iter, Function, Promise};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
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
pub async fn blend_images(
    top: File,
    bot: File,
    algorithm: Option<String>,
    is_inline: Option<bool>,
) -> Result<ImageData, JsValue> {
    let top = JsFuture::from(load_image(top)).await?;
    let bot = JsFuture::from(load_image(bot)).await?;

    let top = get_image_data(top.into())?;
    let bot = get_image_data(bot.into())?;

    blend_images_data(top, bot, algorithm, is_inline)
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
        let (algorithm, params) = pair.1;
        let demultiply = is_algorithm_multiplied(&algorithm);
        let algorithm_fn = get_blending_algorithm(&algorithm);

        if demultiply {
            demultiply_image(&mut current_layer);
        }

        blending::blend_images(&current_layer, &mut composition, &algorithm_fn, params);
    }

    let composition_bytes = &mut composition.to_vec();
    let clamped_composition_bytes: Clamped<&mut [u8]> = Clamped(composition_bytes);
    let result = ImageData::new_with_u8_clamped_array_and_sh(clamped_composition_bytes, composition.width(), composition.height())?;
    Ok(result)
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

fn build_algorithm(algorithm: &String) -> Result<BlendAlgorithm, PConvertError> {
    match BlendAlgorithm::from_str(&algorithm) {
        Ok(algorithm) => Ok(algorithm),
        Err(algorithm) => Err(PConvertError::ArgumentError(format!(
            "Invalid algorithm '{}'",
            algorithm
        ))),
    }
}

#[derive(Serialize, Deserialize)]
struct JSONParams {
    algorithm: String,
    params: HashMap<String, Value>,
}

fn build_params(
    algorithms: Box<[JsValue]>,
) -> Result<Vec<(BlendAlgorithm, Option<BlendAlgorithmParams>)>, PConvertError> {
    let mut result = Vec::new();

    for i in 0..algorithms.len() {
        let element = &algorithms[i];
        if element.is_string() {
            let algorithm =
                build_algorithm(&element.as_string().unwrap_or("multiplicative".to_string()))?;

            result.push((algorithm, None));
        } else if element.is_object() {
            let params: JSONParams = element.into_serde::<JSONParams>().unwrap();
            let algorithm = build_algorithm(&params.algorithm)?;

            let mut blending_params = BlendAlgorithmParams::new();
            for (param_name, param_value) in params.params {
                let param_value: ParamValue = param_value.into();
                blending_params.insert(param_name, param_value);
            }

            result.push((algorithm, Some(blending_params)));
        }
    }

    Ok(result)
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

impl From<Value> for ParamValue {
    fn from(value: Value) -> ParamValue {
        match value {
            Value::Bool(boolean) => ParamValue::Bool(boolean),
            Value::String(string) => ParamValue::Str(string),
            Value::Number(number) => {
                if number.is_f64() {
                    ParamValue::Float(number.as_f64().unwrap())
                } else if number.is_i64() {
                    ParamValue::Long(number.as_i64().unwrap())
                } else {
                    ParamValue::Invalid
                }
            }
            _ => ParamValue::Invalid,
        }
    }
}
