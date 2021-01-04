//! Blend algorithms and PConvert API optional parameter types (`BlendAlgorithmParams` and `Options`, respectively).
//! Low level layer for the composition system.

use std::collections::HashMap;

/// Map of blending algorithm properties and corresponding values.
pub type BlendAlgorithmParams = HashMap<String, Value>;

/// Map of API options and corresponding values.
#[cfg(not(target_arch = "wasm32"))]
pub type Options = HashMap<String, Value>;

/// Abstract data type that can assume multiple primitive types.
/// The data structure is going to be used in the passing of parameters
/// between heterogenous type systems (eg: different VMs).
#[derive(Clone, Debug)]
pub enum Value {
    Bool(bool),
    Long(i64),
    Float(f64),
    Str(String),

    #[cfg(not(feature = "wasm-extension"))]
    Int(i32),

    #[cfg(feature = "wasm-extension")]
    Invalid,
}
