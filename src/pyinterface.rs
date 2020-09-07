use pyo3::prelude::*;
use pyo3::types::PyTuple;
use pyo3::wrap_pyfunction;

#[pyfunction]
fn blend_images(bot_path: String, top_path: String, target_path: String, algorithm: Option<String>, is_inline: Option<bool>) {
    let algorithm = match algorithm {
        Some(algorithm) => algorithm,
        None => {
            let default = "multiplicative";
            println!("Algorithm not specified, using default: {}", default);
            String::from(default)
        }
    };

    let is_inline = match is_inline {
        Some(is_inline) => is_inline,
        None => {
            let default = false;
            println!("Indication to use inline methods not specified, using default: {}", default);
            default
        }
    };

    println!("Blend images: {} - {} - {} - {:?} - {:?}", bot_path, top_path, target_path, algorithm, is_inline);
}

#[pyfunction]
fn blend_multiple(img_paths: &PyTuple, out_path: String) {
    println!("Blend multiple images: {:?} to {}", img_paths, out_path);
}

#[pymodule]
fn pconvert_rust(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(blend_images))?;
    m.add_wrapped(wrap_pyfunction!(blend_multiple))?;
    Ok(())
}
