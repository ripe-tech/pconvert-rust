use std::collections::HashMap;

pub type BlendAlgorithmParams = HashMap<String, ParamValue>;

#[derive(Clone, Debug)]
pub enum ParamValue {
    Bool(bool),
    Int(i32),
    Long(i64),
    Float(f64),
    Str(String),
    Invalid,
}
