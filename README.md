# P(NG)Convert Rust

The [Rust](https://www.rust-lang.org) version of the famous [P(NG)Convert](https://github.com/hivesolutions/pconvert) from Hive Solutions.

This Rust crate can be used as a **command line application**, as a **crate** in another rust project, as a **Web Assembly module** (able to be used within JavaScript that targets web browsers) or as a **python package**.

# Command Line Application

## Compiling & Executing

Build and run with:

```bash
cargo run
```

Alternatively, compile first with:

```bash
cargo build
```

and then run the binary with:

```bash
./target/debug/pconvert-rust
```

Additionally, for better code optimization, compile with the `--release` flag:

```bash
cargo build --release
```

and then run the release binary with:

```bash
./target/release/pconvert-rust
```

## Usage

```console
$ pconvert-rust
Usage: pconvert-rust <command> [args...]
where command can be one of the following: compose, convert, benchmark, version
```

```console
$ pconvert-rust compose <dir>
```

```console
$ pconvert-rust convert <file_in> <file_out>
```

```console
$ pconvert-rust benchmark <dir> [--parallel]
```

```console
$ pconvert-rust version
```

# WebAssembly (WASM) Module

## Compiling & Executing

Follow [this guide](https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_wasm) on how to install `wasm-pack`.

To build, use the `wasm-extension` feature:

```bash
wasm-pack build -- --features wasm-extension
```

To run the demo, follow [this](https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_wasm#Making_our_package_availabe_to_npm).

## Usage

Check the [demo site](examples/wasm/index.js) to see how to use the PConvert WASM module.

JavaScript API exposed:
```javascript
// blends two File objects and returns a File object
blendImages(top, bot, target_file_name, algorithm, is_inline, options)

// blends two ImageData objects and returns an ImageData object
blendImagesData(top, bot, algorithm, is_inline, options)

// blends multiple File objects and returns a File object
blendMultiple(image_files, target_file_name, algorithm, algorithms, is_inline, options)

// blends multiple ImageData objects  and returns an ImageData object
blendMultipleData(images, algorithm, algorithms, is_inline, options)

// returns a JSON of module constants (e.g. ALGORITHMS, FILTER_TYPES, COMPILER_VERSION, ...)
getModuleConstants()

// benchmarks and prints to console various times for different combinations of blending algorithms, compression algorithms and filters for `blendImages`
blendImagesBenchmarkAll(top, bot, is_inline)

// benchmarks and prints to console various times for different combinations of blending algorithms, compression algorithms and filters for `blendMultiple`
blendMultipleBenchmarkAll(image_files, is_inline)
```

# Python package

## Compiling & Executing

This crate can be installed as a python package through the use of `pip`. Simply run:

```bash
pip install .
```

## Usage

Check [this folder](examples/python/) for examples.

Import the python package with:

```python
import pconvert_rust
```

Python API exposed. The parameter `options` is a python dictionary of optional parameters and if `num_threads` is specified with a value of 1 or more, the work load will be distributed across multiple threads (belonging to a internally managed thread pool).

```python
# blends two images read from the local file system and writes the result to the file system
blend_images(bot_path, top_path, target_path, algorithm, is_inline, options)

# blends multiple images read from the local file system and writes the result to the file system
blend_multiple(img_paths, out_path, algorithm, algorithms, is_inline, options)

# returns a python dict with summary information about the internal thread pool (size, active jobs, queued jobs)
get_thread_pool_status()

# access module constants (e.g. ALGORITHMS, FILTER_TYPES, COMPILER_VERSION, ...)
pconvert_rust.ALGORITHMS
pconvert_rust.FILTER_TYPES
pconvert_rust.COMPILER_VERSION
```

## Documentation

Generate with

```bash
cargo doc --lib --all-features
```

## License

P(NG)Convert Rust is currently licensed under the [Apache License, Version 2.0](http://www.apache.org/licenses/).

## Build Automation

[![Build Status](https://travis-ci.org/ripe-tech/pconvert-rust.svg?branch=master)](https://travis-ci.org/ripe-tech/pconvert-rust)
[![Build Status GitHub](https://github.com/ripe-tech/pconvert-rust/workflows/Main%20Workflow/badge.svg)](https://github.com/ripe-tech/pconvert-rust/actions)
[![PyPi Status](https://img.shields.io/pypi/v/pconvert-rust.svg)](https://pypi.python.org/pypi/pconvert-rust)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](https://www.apache.org/licenses/)
