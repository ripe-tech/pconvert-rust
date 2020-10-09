use std::collections::HashMap;

pub type BlendAlgorithmParams = HashMap<String, Value>;
pub type Options = HashMap<String, Value>;

#[derive(Clone, Debug)]
pub enum Value {
    Bool(bool),
    Long(i64),
    Float(f64),
    Str(String),

    #[cfg(not(target_arch = "wasm32"))]
    Int(i32),

    #[cfg(target_arch = "wasm32")]
    Invalid,
}
