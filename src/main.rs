use std::env;

fn print_usage() {
    println!(
        "Usage: pconvert-rust <command> [args...]\nwhere command can be one of the following: compose, convert, benchmark, opencl, version"
    )
}

fn main() {
    let mut args = env::args();
    args.next(); // skip program name

    match args.next() {
        Some(action) => match &action[..] {
            "compose" => pconvert_rust::pcompose(),
            "convert" => pconvert_rust::pconvert(),
            "benchmark" => pconvert_rust::pbenchmark(),
            "opencl" => pconvert_rust::popencl(),
            "version" => pconvert_rust::pversion(),
            _ => print_usage(),
        },
        _ => print_usage(),
    }
}
