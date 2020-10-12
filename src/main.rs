mod cli;

use cli::{pbenchmark, pcompose, pconvert, print_usage, pversion};
use pconvert_rust::errors::PConvertError;
use std::env;

fn main() -> Result<(), PConvertError> {
    let mut args = env::args();

    // skips program name, facilitating the parsing
    // of the extra argument from command line
    args.next();

    match args.next() {
        Some(action) => match &action[..] {
            "convert" => pconvert(&mut args)?,
            "compose" => pcompose(&mut args)?,
            "benchmark" => pbenchmark(&mut args)?,
            "version" => pversion(),
            _ => print_usage(),
        },
        None => print_usage(),
    };

    Ok(())
}
