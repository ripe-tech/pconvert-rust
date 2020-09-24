use crate::blending::params::{BlendAlgorithmParams, Value};
use crate::blending::BlendAlgorithm;
use crate::errors::PConvertError;
use pyo3::prelude::*;
use pyo3::types::{PyBool, PyFloat, PyInt, PyLong, PySequence, PyString};
use std::str::FromStr;

pub fn build_algorithm(algorithm: &String) -> Result<BlendAlgorithm, PyErr> {
    match BlendAlgorithm::from_str(algorithm) {
        Ok(algorithm) => Ok(algorithm),
        Err(algorithm) => Err(PyErr::from(PConvertError::ArgumentError(format!(
            "ArgumentError: invalid algorithm '{}'",
            algorithm
        )))),
    }
}

pub fn build_params(
    algorithms: &PySequence,
) -> Result<Vec<(BlendAlgorithm, Option<BlendAlgorithmParams>)>, PyErr> {
    let mut result = Vec::new();

    for i in 0..algorithms.len()? {
        let element = algorithms.get_item(i)?;

        if let Ok(string) = element.cast_as::<PyString>() {
            let algorithm = build_algorithm(&string.to_string()?.into_owned())?;
            result.push((algorithm, None));
        } else if let Ok(sequence) = element.cast_as::<PySequence>() {
            let algorithm = sequence.get_item(0)?.extract::<String>()?;
            let algorithm = build_algorithm(&algorithm)?;

            let mut blending_params = BlendAlgorithmParams::new();
            let params_sequence = sequence.get_item(1)?;
            if let Ok(params_sequence) = params_sequence.cast_as::<PySequence>() {
                for j in 0..params_sequence.len()? {
                    if let Ok(property_value) = params_sequence.get_item(j)?.cast_as::<PySequence>()
                    {
                        let param_name = property_value.get_item(0)?.extract::<String>()?;
                        let param_value = property_value.get_item(1)?;
                        let param_value = param_value.extract::<Value>()?;
                        blending_params.insert(param_name, param_value);
                    }
                }
            } else {
                return Err(PyErr::from(PConvertError::ArgumentError(
                    "Parameters should be given as a python sequence object".to_string(),
                )));
            }

            result.push((algorithm, Some(blending_params)));
        }
    }

    Ok(result)
}
