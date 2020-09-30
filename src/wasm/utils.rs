use crate::blending::params::{BlendAlgorithmParams, Value};
use crate::blending::BlendAlgorithm;
use crate::errors::PConvertError;
use crate::utils::{decode_png, encode_png};
use crate::wasm::conversions::JSONParams;
use image::png::{CompressionType, FilterType};
use image::{ImageBuffer, Rgba};
use js_sys::{Array, Uint8Array};
use std::str::FromStr;
use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use wasm_bindgen_futures::JsFuture;
use web_sys::{File, ImageData};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

pub async fn load_png(
    file: File,
    demultiply: bool,
) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>, JsValue> {
    let array_buffer = JsFuture::from(file.array_buffer()).await?;
    let uint8_array = Uint8Array::new(&array_buffer);
    let png = decode_png(&uint8_array.to_vec()[..], demultiply)?;
    Ok(png)
}

pub fn encode_file(
    image_buffer: ImageBuffer<Rgba<u8>, Vec<u8>>,
    compression: CompressionType,
    filter: FilterType,
    target_file_name: String,
) -> Result<File, JsValue> {
    let mut encoded_data = Vec::<u8>::with_capacity(image_buffer.to_vec().capacity());
    encode_png(&mut encoded_data, &image_buffer, compression, filter)?;

    unsafe {
        let array_buffer = Uint8Array::view(&encoded_data);
        File::new_with_u8_array_sequence(&Array::of1(&array_buffer), &target_file_name)
    }
}

pub fn encode_image_data(
    image_buffer: ImageBuffer<Rgba<u8>, Vec<u8>>,
    compression: CompressionType,
    filter: FilterType,
) -> Result<ImageData, JsValue> {
    let (width, height) = image_buffer.dimensions();

    let mut encoded_data = Vec::<u8>::with_capacity(image_buffer.to_vec().capacity());
    encode_png(
        &mut encoded_data,
        &image_buffer,
        compression,
        filter,
    )?;

    let bytes = &mut image_buffer.to_vec();
    let clamped_bytes: Clamped<&mut [u8]> = Clamped(bytes);

    ImageData::new_with_u8_clamped_array_and_sh(clamped_bytes, width, height)
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
                let param_value: Value = param_value.into();
                blending_params.insert(param_name, param_value);
            }

            result.push((algorithm, Some(blending_params)));
        }
    }

    Ok(result)
}

pub fn log_benchmark(algorithm: String, blend_time: f64, read_time: f64, write_time: f64) {
    console_log!(
        "{:<20}{:<20}",
        algorithm,
        format!(
            "{}ms (blend {}ms, read {}ms, write {}ms)",
            read_time + blend_time + write_time,
            blend_time,
            read_time,
            write_time
        )
    );
}
