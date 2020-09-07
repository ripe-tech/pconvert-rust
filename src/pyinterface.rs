use pyo3::prelude::*;
use pyo3::types::PyTuple;

use super::blending::{
    blend_images, get_blending_algorithm, is_algorithm_multiplied, BlendAlgorithm,
};

use super::utils::{read_png, write_png};
use std::str::FromStr;

#[pymodule]
fn pconvert_rust(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add("VERSION", env!("CARGO_PKG_VERSION"))?;
    m.add("ALGORITHMS", BlendAlgorithm::all())?;

    #[pyfn(m, "blend_images")]
    fn blend_images_py(
        bot_path: String,
        top_path: String,
        target_path: String,
        algorithm: Option<String>,
        is_inline: Option<bool>,
    ) {
        let algorithm = match algorithm {
            Some(algorithm) => BlendAlgorithm::from_str(&algorithm).unwrap(),
            None => BlendAlgorithm::from_str("multiplicative").unwrap(),
        };

        //TODO: actually make use of this
        let _is_inline = match is_inline {
            Some(is_inline) => is_inline,
            None => false,
        };

        let demultiply = is_algorithm_multiplied(&algorithm);
        let algorithm_fn = get_blending_algorithm(&algorithm);

        let mut bot = read_png(bot_path, demultiply);
        let top = read_png(top_path, demultiply);
        blend_images(&top, &mut bot, &algorithm_fn);

        write_png(target_path, &bot);
    }

    #[pyfn(m, "blend_multiple")]
    fn blend_multiple_py(img_paths: &PyTuple, out_path: String) {
        if img_paths.len() < 1 {
            eprintln!("ERROR: Specify at least one image path");
            std::process::exit(-1);
        }

        let algorithm = BlendAlgorithm::Multiplicative;
        let demultiply = is_algorithm_multiplied(&algorithm);
        let algorithm_fn = get_blending_algorithm(&algorithm);

        let mut paths_iter = img_paths.iter();
        let mut composition = read_png(paths_iter.next().unwrap().extract().unwrap(), demultiply);
        while let Some(path) = paths_iter.next() {
            let current = read_png(path.extract().unwrap(), demultiply);
            blend_images(&current, &mut composition, &algorithm_fn);
        }
        write_png(out_path, &composition);
    }

    Ok(())
}
