use pyo3::exceptions::{AttributeError, Exception, IOError, NotImplementedError};
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