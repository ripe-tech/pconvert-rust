# Modules overview

## Source files

* [`main.rs`](./main.rs)
  * exposes the simple CLI

* [`cli.rs`](./cli.rs)
  * CLI functions to allow testing pconvert binary

* [`lib.rs`](./lib.rs)
  * exports blending functions to be used by other Rust crates
  * conditionally imports submodules depending on target environment (e.g. WASM does not make use of the submodule `pymodule`)

* [`benchmark.rs`](./benchmark.rs)
  * benchmark struct and associated functions

* [`errors.rs`](./errors.rs)
  * pconvert errors definition
  * some external errors to pconvert errors conversion

* [`parallelism.rs`](./parallelism.rs)
  * thread pool definition
  * worker threads definition
  * thread pool status definition

* [`utils.rs`](./utils.rs)
  * decode/encode PNG functions
  * read/write from file system PNG functions
  * external crate type conversions
  * other utility functions

## Submodules

* [`blending`](./blending/)
  * blending algorithms

* [`pymodule`](./pymodule/)
  * exposure of pconvert's API as a python module

* [`wasm`](./wasm/)
  * exposure of pconvert's API as a Web Assembly (WASM) module
