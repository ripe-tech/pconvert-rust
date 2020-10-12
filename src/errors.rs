//! Internal error types definition and external error type conversions

use image::error::ImageError;
use std::error::Error;
use std::fmt::{Display, Formatter, Result};
use std::io;

/// Error types used across this crate
#[derive(Debug)]
pub enum PConvertError {
    ArgumentError(String),
    UnsupportedImageTypeError,
    IOError(io::Error),
    ImageLibError(ImageError),
}

impl Display for PConvertError {
    fn fmt(&self, formatter: &mut Formatter) -> Result {
        match &*self {
            PConvertError::UnsupportedImageTypeError => write!(
                formatter,
                "UnsupportedImageTypeError: images should be PNGs encoded as RGBA8"
            ),
            PConvertError::ImageLibError(err) => err.fmt(formatter),
            PConvertError::IOError(err) => err.fmt(formatter),
            PConvertError::ArgumentError(msg) => write!(formatter, "{}", msg),
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
