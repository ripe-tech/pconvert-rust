//! PNG decode/encode and load functions,
//! console log macros,
//! argument parsing from javascript input to inner-crate rust types
//! and other utility functions

use crate::blending::params::{BlendAlgorithmParams, Value};
use crate::blending::BlendAlgorithm;
use crate::errors::PConvertError;
use crate::utils::{decode_png, encode_png};
use crate::utils::{image_compression_from, image_filter_from};
use crate::wasm::conversions::JSONParams;
use image::codecs::png::{CompressionType, FilterType};
use image::{ImageBuffer, Rgba};
use js_sys::{Array, Uint8Array};
use serde_json::Value as JSONValue;
use std::collections::HashMap;
use std::str::FromStr;
use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use wasm_bindgen_futures::JsFuture;
use web_sys::{File, ImageData};

#[wasm_bindgen]
extern "C" {
    /// JavaScript `console.log` function
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

/// Receives a `File` and returns the decoded PNG byte buffer
pub async fn load_png(
    file: File,
    demultiply: bool,
) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>, JsValue> {
    let array_buffer = JsFuture::from(file.array_buffer()).await?;
    let uint8_array = Uint8Array::new(&array_buffer);
    let png = decode_png(&uint8_array.to_vec()[..], demultiply)?;
    Ok(png)
}

/// Receives png buffer data and encodes it as a `File` with specified `CompressionType` and `FilterType`
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

/// Receives png buffer data and encodes it as an `ImageData` object with specified `CompressionType` and `FilterType`
pub fn encode_image_data(
    image_buffer: ImageBuffer<Rgba<u8>, Vec<u8>>,
    compression: CompressionType,
    filter: FilterType,
) -> Result<ImageData, JsValue> {
    let (width, height) = image_buffer.dimensions();

    let mut encoded_data = Vec::<u8>::with_capacity(image_buffer.to_vec().capacity());
    encode_png(&mut encoded_data, &image_buffer, compression, filter)?;

    let bytes = &mut image_buffer.to_vec();
    let clamped_bytes: Clamped<&mut [u8]> = Clamped(bytes);

    ImageData::new_with_u8_clamped_array_and_sh(clamped_bytes, width, height)
}

/// Attempts to parse a `&String` to a `BlendAlgorithm`.
/// Returns the enum variant if it suceeds. Otherwise it returns a `PConvertError`.
pub fn build_algorithm(algorithm: &String) -> Result<BlendAlgorithm, PConvertError> {
    match BlendAlgorithm::from_str(&algorithm) {
        Ok(algorithm) => Ok(algorithm),
        Err(algorithm) => Err(PConvertError::ArgumentError(format!(
            "Invalid algorithm '{}'",
            algorithm
        ))),
    }
}

/// Attempts to build a vector of blending operations and extra parameters.
/// One pair per blending operation. Returns a `PConvertError` if it fails parsing.
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

/// Retrieves the `image::codecs::png::CompressionType` value from the `HashMap<String, JSONValue>` map if it exists.
/// Otherwise it returns the default value: `CompressionType::Fast`.
pub fn get_compression_type(options: &Option<HashMap<String, JSONValue>>) -> CompressionType {
    options.as_ref().map_or(CompressionType::Fast, |options| {
        options
            .get("compression")
            .map_or(CompressionType::Fast, |compression| match compression {
                JSONValue::String(compression) => image_compression_from(compression.to_string()),
                _ => CompressionType::Fast,
            })
    })
}

/// Retrieves the `image::codecs::png::FilterType` value from the `HashMap<String, JSONValue>` map if it exists.
/// Otherwise it returns the default value: `FilterType::NoFilter`.
pub fn get_filter_type(options: &Option<HashMap<String, JSONValue>>) -> FilterType {
    options.as_ref().map_or(FilterType::NoFilter, |options| {
        options
            .get("filter")
            .map_or(FilterType::NoFilter, |filter| match filter {
                JSONValue::String(filter) => image_filter_from(filter.to_string()),
                _ => FilterType::NoFilter,
            })
    })
}

/// Logs the header/column names of the benchmarks table to the browser console (with `console.log`)
pub fn log_benchmark_header() {
    console_log!(
        "{:<20}{:<20}{:<20}{:<20}",
        "Algorithm",
        "Compression",
        "Filter",
        "Times"
    );
}

/// Logs one line (algorithm, compression, filter, blend time, read time, write time) of the benchmarks table to the browser console (with `console.log`)
pub fn log_benchmark(
    algorithm: String,
    compression: CompressionType,
    filter: FilterType,
    blend_time: f64,
    read_time: f64,
    write_time: f64,
) {
    console_log!(
        "{:<20}{:<20}{:<20}{:<20}",
        algorithm,
        format!("{:#?}", compression),
        format!("{:#?}", filter),
        format!(
            "{}ms (blend {}ms, read {}ms, write {}ms)",
            read_time + blend_time + write_time,
            blend_time,
            read_time,
            write_time
        )
    );
}
