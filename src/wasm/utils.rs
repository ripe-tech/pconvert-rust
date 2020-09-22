use super::jstypes::JSONParams;
use crate::blending::params::{BlendAlgorithmParams, ParamValue};
use crate::blending::BlendAlgorithm;
use crate::errors::PConvertError;
use js_sys::{Function, Promise};
use std::str::FromStr;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{
    window, CanvasRenderingContext2d, File, HtmlCanvasElement, HtmlImageElement, ImageData, Url,
};

pub fn load_image(file: File) -> Promise {
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

pub fn get_image_data(img: HtmlImageElement) -> Result<ImageData, JsValue> {
    let document = window().unwrap().document().unwrap();
    let canvas = document.create_element("canvas").unwrap();
    let canvas: HtmlCanvasElement = canvas
        .dyn_into::<HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();
    canvas.set_width(img.width());
    canvas.set_height(img.height());
    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();
    context.draw_image_with_html_image_element(&img, 0.0, 0.0)?;
    context.get_image_data(0.0, 0.0, img.width().into(), img.height().into())
}

pub fn build_algorithm(algorithm: &String) -> Result<BlendAlgorithm, PConvertError> {
    match BlendAlgorithm::from_str(&algorithm) {
        Ok(algorithm) => Ok(algorithm),
        Err(algorithm) => Err(PConvertError::ArgumentError(format!(
            "Invalid algorithm '{}'",
            algorithm
        ))),
    }
}

pub fn build_params(
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
