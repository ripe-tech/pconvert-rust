use std::env;

fn print_usage() {
    println!(
        "Usage: pconvert-rust <command> [args...]\nwhere command can be one of the following: compose, convert, benchmark, opencl, version"
    );
}

fn main() {
    let mut args = env::args();
    args.next(); // skip program name

    match args.next() {
        Some(action) => match &action[..] {
            "convert" => pconvert_rust::pconvert(args),
            "compose" => pconvert_rust::pcompose(args),
            _ => print_usage(),
        },
        None => print_usage(),
    }
}
