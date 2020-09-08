use chrono::Utc;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;

const BUILD_OUT_FILE: &str = "constants.rs";
const SOURCE_DIR: &str = "./src";

fn main() {
    let dest_path = Path::new(SOURCE_DIR).join(Path::new(BUILD_OUT_FILE));
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
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
}

fn write_str_constant_to_file(file: &mut File, key: &str, val: &str) {
    writeln!(file, "pub const {}: &str = \"{}\";", key, val).expect(&format!(
        "Failed to write '{}' to 'build_constants.rs'",
        key
    ));
}
