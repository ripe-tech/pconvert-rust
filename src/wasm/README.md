# Web Assembly (WASM)

Source files:

- [`mod.rs`](./mod.rs)
    - exposes the WASM module API to be used within JavaScript files that target the web browsers

- [`benchmark.rs`](./benchmark.rs)
    - exposes the WASM module benchmark API to be used within JavaScript files that target the web browsers
    - prints measured times to the console

- [`conversions.rs`](./conversions.rs)
    - type conversions from and to JavaScript types

- [`utils.rs`](./utils.rs)
    - `console.log` macro
    - decode/encode PNG functions
    - other utility functions

## JavaScript Web API

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
