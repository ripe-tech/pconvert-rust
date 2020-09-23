# P(NG)Convert Rust

The [Rust](https://www.rust-lang.org) version of the famous [P(NG)Convert](https://github.com/hivesolutions/pconvert) from Hive Solutions.

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

## WebAssembly (WASM)

### Compiling
1. Install [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)

2. Build the WASM package with:
    ```bash
    wasm-pack build --release
    ```
    The resultant WASM package is in `pkg/` 

### Using the WASM package

The WASM package can be used by importing:

```js
const js = import("pconvert_rust.js");
```

## License

P(NG)Convert Rust is currently licensed under the [Apache License, Version 2.0](http://www.apache.org/licenses/).

## Build Automation

[![Build Status](https://travis-ci.org/ripe-tech/pconvert-rust.svg?branch=master)](https://travis-ci.org/ripe-tech/pconvert-rust)
[![Build Status GitHub](https://github.com/ripe-tech/pconvert-rust/workflows/Main%20Workflow/badge.svg)](https://github.com/ripe-tech/pconvert-rust/actions)
[![PyPi Status](https://img.shields.io/pypi/v/pconvert-rust.svg)](https://pypi.python.org/pypi/pconvert-rust)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](https://www.apache.org/licenses/)
