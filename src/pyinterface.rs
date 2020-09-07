use pyo3::prelude::*;
use pyo3::types::PyTuple;
use pyo3::wrap_pyfunction;

#[pyfunction]
fn blend_images(img1_path: String, img2_path: String, out_path: String) {
    println!("Blend images: {} - {} - {}", img1_path, img2_path, out_path);
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
