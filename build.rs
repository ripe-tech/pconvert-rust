/// Build script (https://doc.rust-lang.org/cargo/reference/build-scripts.html)
/// This script is executed as the first step in the compilation process.
/// Here we export metadata constants to a `constants.rs` file which is then imported and used by the remaining crate.
///
/// # Examples
///
/// In C you can use the preprocessor macro `__DATE__` to save the compilation date like:
///
/// ```c
/// #define PCONVERT_COMPILATION_DATE __DATE__
/// ```
///
/// Rust does not have such preprocessor macros, so we use this script and do:
///
/// ```rust
/// let now_utc = chrono::Utc::now();
/// write_str_constant_to_file(
///     &mut file,
///     "COMPILATION_DATE",
///     &format!("{}", now_utc.format("%b %d %Y")),
/// );
/// ```
use chrono::Utc;
use image::png::{CompressionType, FilterType};
use num_cpus;
use regex::Regex;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::process::Command;
use std::str;

const BUILD_OUT_FILE: &str = "constants.rs";
const SOURCE_DIR: &str = "./src";
const TOML: &'static str = include_str!("Cargo.toml");

fn main() {
    let dest_path = Path::new(SOURCE_DIR).join(Path::new(BUILD_OUT_FILE));
    let mut file = OpenOptions::new()
        .truncate(true)
        .write(true)
        .create(true)
        .open(dest_path)
        .expect(&format!("Can't open '{}'", BUILD_OUT_FILE));

    let module_doc_string = "//! Global constants, such as compiler version used, algorithms, compression and filters supported and others";
    writeln!(file, "{}", module_doc_string).unwrap();

    let now_utc = Utc::now();
    write_str_constant_to_file(
        &mut file,
        "COMPILATION_DATE",
        &format!("{}", now_utc.format("%b %d %Y")),
    );
    write_str_constant_to_file(
        &mut file,
        "COMPILATION_TIME",
        &format!("{}", now_utc.format("%H:%M:%S")),
    );

    write_str_constant_to_file(
        &mut file,
        "VERSION",
        option_env!("CARGO_PKG_VERSION").unwrap_or("UNKNOWN"),
    );

    write_vec_constant_to_file(
        &mut file,
        "ALGORITHMS",
        vec![
            "alpha",
            "multiplicative",
            "source_over",
            "destination_over",
            "mask_top",
            "first_top",
            "first_bottom",
            "disjoint_over",
            "disjoint_under",
            "disjoint_debug",
        ],
    );

    write_str_constant_to_file(&mut file, "COMPILER", "rustc");

    let compiler_version = Command::new("rustc")
        .arg("--version")
        .output()
        .ok()
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .unwrap_or("UNKNOWN".to_string());
    let re = Regex::new("rustc ([\\d.\\d.\\d]*)").unwrap();
    let compiler_version = re
        .captures(&compiler_version)
        .unwrap()
        .get(1)
        .unwrap()
        .as_str();
    write_str_constant_to_file(&mut file, "COMPILER_VERSION", &compiler_version);

    let re = Regex::new("image.*version = \"(.*)\",").unwrap();
    let libpng_version = format!(
        "image-{}",
        re.captures(TOML).unwrap().get(1).unwrap().as_str()
    );
    write_str_constant_to_file(&mut file, "LIBPNG_VERSION", &libpng_version);

    let mut features = vec!["cpu"];

    if cfg!(feature = "wasm-extension") {
        features.push("wasm")
    }

    if cfg!(feature = "python-extension") {
        features.push("python")
    }

    write_vec_constant_to_file(&mut file, "FEATURES", features);

    write_str_constant_to_file(
        &mut file,
        "PLATFORM_CPU_BITS",
        &(std::mem::size_of::<usize>() * 8).to_string(),
    );

    let libpng_filter_types = vec![
        FilterType::NoFilter,
        FilterType::Avg,
        FilterType::Paeth,
        FilterType::Sub,
        FilterType::Up,
    ];
    write_enum_variants_to_file(&mut file, "FILTER_TYPES", libpng_filter_types);

    let libpng_compression_types = vec![
        CompressionType::Default,
        CompressionType::Best,
        CompressionType::Fast,
        CompressionType::Huffman,
        CompressionType::Rle,
    ];
    write_enum_variants_to_file(&mut file, "COMPRESSION_TYPES", libpng_compression_types);

    write_constant_to_file(&mut file, "DEFAULT_THREAD_POOL_SIZE", num_cpus::get());

    write_constant_to_file(&mut file, "MAX_THREAD_POOL_SIZE", num_cpus::get() * 10);
}

fn write_constant_to_file<T>(file: &mut File, key: &str, val: T)
where
    T: std::fmt::Display,
{
    writeln!(
        file,
        "pub const {}: {} = {};",
        key,
        std::any::type_name::<T>(),
        val
    )
    .expect(&format!(
        "Failed to write '{}' to 'build_constants.rs'",
        key
    ));
}

fn write_str_constant_to_file(file: &mut File, key: &str, val: &str) {
    writeln!(file, "pub const {}: &str = \"{}\";", key, val).expect(&format!(
        "Failed to write '{}' to 'build_constants.rs'",
        key
    ));
}

fn write_vec_constant_to_file<T>(file: &mut File, key: &str, vec: Vec<T>)
where
    T: std::fmt::Display,
{
    let mut list_str = String::new();
    for value in &vec {
        list_str.push_str(&format!("\"{}\",", value))
    }
    list_str.pop();
    writeln!(
        file,
        "pub const {}: [{}; {}] = [{}];",
        key,
        std::any::type_name::<T>(),
        vec.len(),
        list_str
    )
    .expect(&format!(
        "Failed to write '{}' to 'build_constants.rs'",
        key
    ));
}

fn write_enum_variants_to_file<T>(file: &mut File, key: &str, vec: Vec<T>)
where
    T: std::fmt::Debug,
{
    let mut list_str = String::new();
    for value in &vec {
        list_str.push_str(&format!("{}::{:?},", std::any::type_name::<T>(), value))
    }
    list_str.pop();
    writeln!(
        file,
        "pub const {}: [{}; {}] = [{}];",
        key,
        std::any::type_name::<T>(),
        vec.len(),
        list_str
    )
    .expect(&format!(
        "Failed to write '{}' to 'build_constants.rs'",
        key
    ));
}
