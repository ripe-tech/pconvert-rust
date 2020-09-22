use std::collections::HashMap;

pub type BlendAlgorithmParams = HashMap<String, ParamValue>;

#[derive(Clone, Debug)]
pub enum ParamValue {
    Bool(bool),
    Long(i64),
    Float(f64),
    Str(String),

    #[cfg(not(target_arch = "wasm32"))]
    Int(i32),
    
    #[cfg(target_arch = "wasm32")]
    Invalid,
}
