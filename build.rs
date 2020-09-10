use chrono::Utc;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::process::Command;
use std::str;

const BUILD_OUT_FILE: &str = "constants.rs";
const SOURCE_DIR: &str = "./src";

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
fn main() {
    let dest_path = Path::new(SOURCE_DIR).join(Path::new(BUILD_OUT_FILE));
    let mut file = OpenOptions::new()
        .truncate(true)
        .write(true)
        .create(true)
        .open(dest_path)
        .expect(&format!("Can't open '{}'", BUILD_OUT_FILE));

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
            "first_top",
            "first_bottom",
            "disjoint_over",
            "disjoint_under",
            "disjoint_debug",
        ],
    );

    write_str_constant_to_file(&mut file, "COMPILER", "rustc");

    let mut compiler_version = Command::new("rustc")
        .arg("--version")
        .output()
        .ok()
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .unwrap_or("UNKNOWN".to_string());

    if compiler_version.ends_with("\n") {
        compiler_version.pop();
    }
    write_str_constant_to_file(&mut file, "COMPILER_VERSION", &compiler_version);

    write_str_constant_to_file(&mut file, "LIBPNG_VERSION", "UNKNOWN");

    write_vec_constant_to_file(&mut file, "FEATURES", vec!["cpu", "python"]);

    write_str_constant_to_file(
        &mut file,
        "PLATFORM_CPU_BITS",
        &(std::mem::size_of::<usize>() * 8).to_string(),
    );
}

fn write_str_constant_to_file(file: &mut File, key: &str, val: &str) {
    writeln!(file, "pub const {}: &str = \"{}\";", key, val).expect(&format!(
        "Failed to write '{}' to 'build_constants.rs'",
        key
    ));
}

fn write_vec_constant_to_file(file: &mut File, key: &str, vec: Vec<&str>) {
    let mut list_str = String::new();
    for value in &vec {
        list_str.push_str(&format!("\"{}\",", value))
    }
    list_str.pop();
    writeln!(
        file,
        "pub const {}: [&'static str; {}] = [{}];",
        key,
        vec.len(),
        list_str
    )
    .expect(&format!(
        "Failed to write '{}' to 'build_constants.rs'",
        key
    ));
}
