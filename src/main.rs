use pconvert_rust::errors::PConvertError;
use std::env;

fn main() -> Result<(), PConvertError> {
    let mut args = env::args();

    // skips program name, facilitating the parsing
    // of the extra argument from command line
    args.next();

    match args.next() {
        Some(action) => match &action[..] {
            "convert" => pconvert_rust::pconvert(&mut args)?,
            "compose" => pconvert_rust::pcompose(&mut args)?,
            "benchmark" => pconvert_rust::pbenchmark(&mut args)?,
            "version" => pconvert_rust::pversion()?,
            _ => print_usage(),
        },
        None => print_usage(),
    };

    Ok(())
}

fn print_usage() {
    println!("Usage: pconvert-rust <command> [args...]\nwhere command can be one of the following: compose, convert, benchmark, opencl, version");
}
