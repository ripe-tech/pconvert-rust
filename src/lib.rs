use std::env;

pub fn pconvert(mut args: env::Args) {
    let file_in = match args.next() {
        Some(name) => name,
        None => {
            println!("Missing input file.\nUsage: pconvert convert <file_in> <file_out>");
            return;
        }
    };

    let _file_out = match args.next() {
        Some(name) => name,
        None => {
            println!("Missing output path.\nUsage: pconvert convert <file_in> <file_out>");
            return;
        }
    };

    //load PNG
    let img = match image::open(file_in).expect("Failed to open input file") {
        image::DynamicImage::ImageRgba8(img) => img,
        _ => {
            println!("aaa");
            return;
        }
    };
    println!("dimensions {:?}", img.dimensions());

    //turn the image blueish: "sets red value to 0 and green value to the blue one (blue filter)"
    //save PNG
}
