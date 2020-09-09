use super::blending::{
    blend_images, get_blending_algorithm, is_algorithm_multiplied, BlendAlgorithm,
};
use super::utils::{read_png, write_png};
use crate::constants;
use image::png::{CompressionType, FilterType};
use pyo3::prelude::*;
use pyo3::types::PySequence;
use std::str::FromStr;

#[pymodule]
fn pconvert_rust(_py: Python, m: &PyModule) -> PyResult<()> {
    /* Module exported constants */
    m.add("COMPILATION_DATE", constants::COMPILATION_DATE)?;

    m.add("COMPILATION_TIME", constants::COMPILATION_TIME)?;

    m.add("VERSION", constants::VERSION)?;

    m.add("ALGORITHMS", constants::ALGORITHMS.to_vec())?;

    m.add("COMPILER", constants::COMPILER)?;

    m.add("COMPILER_VERSION", constants::COMPILER_VERSION)?;

    m.add("LIBPNG_VERSION", constants::LIBPNG_VERSION)?;

    m.add("FEATURES", constants::FEATURES.to_vec())?;

    #[pyfn(m, "blend_images")]
    fn blend_images_py(
        bot_path: String,
        top_path: String,
        target_path: String,
        algorithm: Option<String>,
        is_inline: Option<bool>,
    ) {
        let algorithm_str = algorithm.unwrap_or(String::from("multiplicative"));
        let algorithm =
            BlendAlgorithm::from_str(&algorithm_str).unwrap_or(BlendAlgorithm::Multiplicative);

        //TODO: actually make use of this
        let _is_inline = is_inline.unwrap_or(false);

        let demultiply = is_algorithm_multiplied(&algorithm);
        let algorithm_fn = get_blending_algorithm(&algorithm);

        let mut bot = read_png(bot_path, demultiply);
        let top = read_png(top_path, demultiply);
        blend_images(&top, &mut bot, &algorithm_fn);

        write_png(
            target_path,
            &bot,
            CompressionType::Fast,
            FilterType::NoFilter,
        );
    }

    #[pyfn(m, "blend_multiple")]
    fn blend_multiple_py(
        img_paths: &PySequence,
        out_path: String,
        algorithm: Option<String>,
        algorithms: Option<Vec<String>>,
        is_inline: Option<bool>,
    ) {
        println!("Printing from Rust code #1");
        let num_images = img_paths.len().unwrap() as usize;

        if num_images < 1 {
            eprintln!("ERROR: Specify at least one image path");
            std::process::exit(-1);
        }

        //TODO: actually make use of this
        let _is_inline = is_inline.unwrap_or(false);

        let algorithms_to_apply: Vec<String>;
        if let Some(algorithms) = algorithms {
            if algorithms.len() != num_images - 1 {
                eprintln!("ERROR: The list of algorithms to apply must be of size {} (one per image blending operation)", num_images - 1);
                std::process::exit(-1);
            } else {
                algorithms_to_apply = algorithms;
            }
        } else if let Some(algorithm) = algorithm {
            algorithms_to_apply = vec![algorithm; num_images - 1]
        } else {
            algorithms_to_apply = vec!["multiplicative".to_owned(); num_images - 1]
        }

        println!("Printing from Rust code #2");

        let mut zip_iter = img_paths.iter().unwrap().zip(algorithms_to_apply.iter());
        let first_pair = zip_iter.next().unwrap();
        let first_path = first_pair.0.unwrap().extract::<String>().unwrap();

        let first_algorithm = first_pair.1;
        let demultiply =
            is_algorithm_multiplied(&BlendAlgorithm::from_str(first_algorithm).expect(&format!(
                "Blending algorithm '{}' does not exist",
                first_algorithm
            )));
        let mut composition = read_png(first_path, demultiply);
        while let Some(pair) = zip_iter.next() {
            let path = pair.0.unwrap().extract::<String>().unwrap();
            let algorithm = pair.1;

            let algorithm = BlendAlgorithm::from_str(algorithm).expect(&format!(
                "Blending algorithm '{}' does not exist",
                algorithm
            ));
            println!("Printing from Rust code loop with algorithm {}", algorithm);
            let demultiply = is_algorithm_multiplied(&algorithm);
            let algorithm_fn = get_blending_algorithm(&algorithm);
            let current_layer = read_png(path, demultiply);
            blend_images(&current_layer, &mut composition, &algorithm_fn);
        }
        write_png(
            out_path,
            &composition,
            CompressionType::Fast,
            FilterType::NoFilter,
        );
        println!("Printing from Rust code #3");
    }

    Ok(())
}
