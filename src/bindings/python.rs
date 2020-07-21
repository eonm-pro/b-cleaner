#[cfg(feature = "stem")]
pub use rust_stemmers::Algorithm;

use crate::cleaners::*;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pyfunction]
fn clean_title(input: Vec<&str>) -> PyResult<Vec<String>> {
    let mut title = TitleCleaner::new(&input);
    title.clean();

    Ok(title.tokens().into_iter().map(|e| e.to_string()).collect::<Vec<String>>())
}

#[pyfunction]
fn clean_author(input: Vec<&str>) -> PyResult<Vec<String>> {
    let mut author = AuthorCleaner::new(&input);
    author.clean();

    Ok(author.tokens().into_iter().map(|e| e.to_string()).collect::<Vec<String>>())
}

#[pymodule]
fn b_cleaner(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(clean_title))?;
    m.add_wrapped(wrap_pyfunction!(clean_author))?;
    
    Ok(())
}