use crate::blending::params::Value;
use crate::errors::PConvertError;
use pyo3::conversion::FromPyObject;
use pyo3::exceptions::TypeError;
use pyo3::exceptions::{AttributeError, Exception, IOError, NotImplementedError};
use pyo3::prelude::*;
use pyo3::types::{PyBool, PyFloat, PyInt, PyLong, PyString};
use pyo3::PyErr;

impl From<PConvertError> for PyErr {
    fn from(err: PConvertError) -> PyErr {
        match err {
            PConvertError::ArgumentError(err) => AttributeError::py_err(err.to_string()),
            PConvertError::ImageLibError(err) => Exception::py_err(err.to_string()),
            PConvertError::UnsupportedImageTypeError => {
                NotImplementedError::py_err(err.to_string())
            }
            PConvertError::IOError(err) => IOError::py_err(err.to_string()),
        }
    }
}

impl FromPyObject<'_> for Value {
    fn extract(ob: &'_ PyAny) -> PyResult<Self> {
        if let Ok(boolean) = ob.cast_as::<PyBool>() {
            let boolean = boolean.is_true();
            Ok(Value::Bool(boolean))
        } else if let Ok(float) = ob.cast_as::<PyFloat>() {
            let float = float.value();
            Ok(Value::Float(float))
        } else if let Ok(int) = ob.cast_as::<PyInt>() {
            let int = int.extract::<i32>()?;
            Ok(Value::Int(int))
        } else if let Ok(long) = ob.cast_as::<PyLong>() {
            let long = long.extract::<i64>()?;
            Ok(Value::Long(long))
        } else if let Ok(string) = ob.cast_as::<PyString>() {
            let string = string.to_string()?.into_owned();
            Ok(Value::Str(string))
        } else {
            let msg = format!("Failure converting {}", ob);
            Err(TypeError::py_err(msg))
        }
    }
}
