//! From and to conversions for rust and javascript types

use crate::blending::params::Value;
use crate::errors::PConvertError;
use serde::{Deserialize, Serialize};
use serde_json::Value as JSONValue;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

/// Blending algorithm and extra parameters for WASM module
#[derive(Serialize, Deserialize)]
pub struct JSONParams {
    pub algorithm: String,
    pub params: HashMap<String, JSONValue>,
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

impl From<JSONValue> for Value {
    fn from(value: JSONValue) -> Value {
        match value {
            JSONValue::Bool(boolean) => Value::Bool(boolean),
            JSONValue::String(string) => Value::Str(string),
            JSONValue::Number(number) => {
                if number.is_f64() {
                    Value::Float(number.as_f64().unwrap())
                } else if number.is_i64() {
                    Value::Long(number.as_i64().unwrap())
                } else {
                    Value::Invalid
                }
            }
            _ => Value::Invalid,
        }
    }
}
