use crate::blending::params::ParamValue;
use crate::errors::PConvertError;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct JSONParams {
    pub algorithm: String,
    pub params: HashMap<String, Value>,
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
