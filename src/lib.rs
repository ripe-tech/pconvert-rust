//! The [Rust](https://www.rust-lang.org) version of the famous [P(NG)Convert](https://github.com/hivesolutions/pconvert) from Hive Solutions.
//! This Rust crate can be used as a **crate** in another rust project, as a **Web Assembly module** (able to be used within JavaScript that targets web browsers) or as a **python package**.
//!
//! # WebAssembly (WASM) Module
//!
//! Follow [this guide](https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_wasm) on how to install `wasm-pack`.
//!
//! To build, use the `wasm-extension` feature:
//!
//! ```console
//! $ wasm-pack build -- --features wasm-extension
//! ```
//!
//! # Python package
//!
//! This crate can be installed as a python package through the use of `pip`. Simply run:
//!
//! ```console
//! $ pip install pconvert-rust/.
//! ```
//!
//! ## License
//!
//! P(NG)Convert Rust is currently licensed under the [Apache License, Version 2.0](http://www.apache.org/licenses/).

pub mod benchmark;
pub mod blending;
pub mod constants;
pub mod errors;
pub mod parallelism;
pub mod utils;

#[cfg(feature = "python-extension")]
pub mod pymodule;

#[cfg(feature = "wasm-extension")]
pub mod wasm;
