//! From and to conversions for rust and python types.

use crate::blending::params::Value;
use crate::errors::PConvertError;
use crate::parallelism::ThreadPoolStatus;
use pyo3::conversion::FromPyObject;
use pyo3::exceptions::{
    PyAttributeError, PyException, PyIOError, PyNotImplementedError, PyTypeError,
};
use pyo3::prelude::*;
use pyo3::types::{IntoPyDict, PyBool, PyDict, PyFloat, PyInt, PyLong, PyString};
use pyo3::PyErr;

impl From<PConvertError> for PyErr {
    fn from(err: PConvertError) -> PyErr {
        match err {
            PConvertError::ArgumentError(err) => PyAttributeError::new_err(err),
            PConvertError::ImageLibError(err) => PyException::new_err(err.to_string()),
            PConvertError::UnsupportedImageTypeError => {
                PyNotImplementedError::new_err(err.to_string())
            }
            PConvertError::IOError(err) => PyIOError::new_err(err.to_string()),
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
            let string = string.to_string();
            Ok(Value::Str(string))
        } else {
            let msg = format!("Failure converting {}", ob);
            Err(PyTypeError::new_err(msg))
        }
    }
}

impl IntoPyDict for ThreadPoolStatus {
    fn into_py_dict(self, py: Python<'_>) -> &PyDict {
        let py_dict = PyDict::new(py);

        py_dict.set_item("size", self.size()).unwrap();
        py_dict.set_item("queued", self.queued()).unwrap();
        py_dict.set_item("active", self.active()).unwrap();

        py_dict
    }
}
