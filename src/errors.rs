use image::error::ImageError;
use pyo3::exceptions::{AttributeError, Exception, IOError, NotImplementedError};
use pyo3::PyErr;
use std::error::Error;
use std::fmt::{Display, Formatter, Result};
use std::io;

#[derive(Debug)]
pub enum PConvertError {
    ArgumentError(String),
    UnsupportedImageTypeError,
    IOError(io::Error),
    ImageLibError(ImageError),
}

impl Display for PConvertError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match &*self {
            PConvertError::UnsupportedImageTypeError => write!(
                f,
                "UnsupportedImageTypeError: images should be PNGs encoded as RGBA8"
            ),
            PConvertError::ImageLibError(err) => err.fmt(f),
            PConvertError::IOError(err) => err.fmt(f),
            PConvertError::ArgumentError(msg) => write!(f, "{}", msg),
        }
    }
}

impl Error for PConvertError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            PConvertError::UnsupportedImageTypeError => None,
            PConvertError::ArgumentError(_) => None,
            PConvertError::ImageLibError(ref err) => Some(err),
            PConvertError::IOError(ref err) => Some(err),
        }
    }
}

impl From<io::Error> for PConvertError {
    fn from(err: io::Error) -> PConvertError {
        PConvertError::IOError(err)
    }
}

impl From<ImageError> for PConvertError {
    fn from(err: ImageError) -> PConvertError {
        PConvertError::ImageLibError(err)
    }
}

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
