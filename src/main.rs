use std::env;

fn print_usage() {
    println!(
        "Usage: pconvert-rust <command> [args...]\nwhere command can be one of the following: compose, convert, benchmark, opencl, version"
    );
}

fn main() {
    let mut args = env::args();
    
    // skip program name, facilitating the parsing
    // of the extra argument from command line
    args.next();

    match args.next() {
        Some(action) => match &action[..] {
            "convert" => pconvert_rust::pconvert(&mut args),
            "compose" => pconvert_rust::pcompose(&mut args),
            _ => print_usage(),
        },
        None => print_usage(),
    }
}
