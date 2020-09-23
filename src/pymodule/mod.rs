mod errors;
mod utils;

use crate::blending::params::BlendAlgorithmParams;
use crate::blending::{
    blend_images, get_blending_algorithm, is_algorithm_multiplied, BlendAlgorithm,
};
use crate::constants;
use crate::errors::PConvertError;
use crate::utils::{read_png, write_png};
use image::png::{CompressionType, FilterType};
use pyo3::prelude::*;
use pyo3::types::PySequence;
use utils::{build_algorithm, build_params};

#[pymodule]
fn pconvert_rust(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add("COMPILATION_DATE", constants::COMPILATION_DATE)?;

    m.add("COMPILATION_TIME", constants::COMPILATION_TIME)?;

    m.add("VERSION", constants::VERSION)?;

    m.add("ALGORITHMS", constants::ALGORITHMS.to_vec())?;

    m.add("COMPILER", constants::COMPILER)?;

    m.add("COMPILER_VERSION", constants::COMPILER_VERSION)?;

    m.add("LIBPNG_VERSION", constants::LIBPNG_VERSION)?;

    m.add("FEATURES", constants::FEATURES.to_vec())?;

    m.add("PLATFORM_CPU_BITS", constants::PLATFORM_CPU_BITS)?;

    #[pyfn(m, "blend_images")]
    fn blend_images_py(
        bot_path: String,
        top_path: String,
        target_path: String,
        algorithm: Option<String>,
        is_inline: Option<bool>,
    ) -> PyResult<()> {
        let algorithm = algorithm.unwrap_or(String::from("multiplicative"));
        let algorithm = build_algorithm(&algorithm)?;

        let _is_inline = is_inline.unwrap_or(false);

        let demultiply = is_algorithm_multiplied(&algorithm);
        let algorithm_fn = get_blending_algorithm(&algorithm);

        let mut bot = read_png(bot_path, demultiply)?;
        let top = read_png(top_path, demultiply)?;
        blend_images(&top, &mut bot, &algorithm_fn, &None);

        write_png(
            target_path,
            &bot,
            CompressionType::Fast,
            FilterType::NoFilter,
        )?;

        Ok(())
    }

    #[pyfn(m, "blend_multiple")]
    fn blend_multiple_py(
        img_paths: &PySequence,
        out_path: String,
        algorithm: Option<String>,
        algorithms: Option<&PySequence>,
        is_inline: Option<bool>,
    ) -> PyResult<()> {
        let num_images = img_paths.len()? as usize;

        if num_images < 1 {
            return Err(PyErr::from(PConvertError::ArgumentError(
                "ArgumentError: 'img_paths' must contain at least one path".to_string(),
            )));
        }

        if algorithms.is_some() && algorithms.unwrap().len()? != num_images as isize - 1 {
            return Err(PyErr::from(PConvertError::ArgumentError(format!(
                "ArgumentError: 'algorithms' must be of size {} (one per blending operation)",
                num_images - 1
            ))));
        };

        let _is_inline = is_inline.unwrap_or(false);

        let algorithms_to_apply: Vec<(BlendAlgorithm, Option<BlendAlgorithmParams>)> =
            if let Some(algorithms) = algorithms {
                build_params(algorithms)?
            } else if let Some(algorithm) = algorithm {
                let algorithm = build_algorithm(&algorithm)?;
                vec![(algorithm, None); num_images - 1]
            } else {
                vec![(BlendAlgorithm::Multiplicative, None); num_images - 1]
            };

        let mut img_paths_iter = img_paths.iter()?;
        let first_path = img_paths_iter.next().unwrap()?.to_string();
        let first_demultiply = is_algorithm_multiplied(&algorithms_to_apply[0].0);
        let mut composition = read_png(first_path, first_demultiply)?;
        let mut zip_iter = img_paths_iter.zip(algorithms_to_apply.iter());
        while let Some(pair) = zip_iter.next() {
            let path = pair.0?.extract::<String>()?;
            let (algorithm, params) = pair.1;
            let demultiply = is_algorithm_multiplied(&algorithm);
            let algorithm_fn = get_blending_algorithm(&algorithm);
            let current_layer = read_png(path, demultiply)?;
            blend_images(&current_layer, &mut composition, &algorithm_fn, params);
        }

        write_png(
            out_path,
            &composition,
            CompressionType::Fast,
            FilterType::NoFilter,
        )?;

        Ok(())
    }

    Ok(())
}