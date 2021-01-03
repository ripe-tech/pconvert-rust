//! Constants that define the current build and execution environment.

#[cfg(feature = "readonly")]
pub mod mock;
#[cfg(feature = "readonly")]
pub use self::mock::*;

#[cfg(not(feature = "readonly"))]
pub mod generated;
#[cfg(not(feature = "readonly"))]
pub use self::generated::*;
