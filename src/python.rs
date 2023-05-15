use pyo3::{Py, pyclass, pymethods, pymodule, PyResult, Python, types::PyModule};

use crate::vtracker::VTracker as RustVTracker;

#[pyclass]
struct VTracker {
    rs: RustVTracker,
}

#[pymethods]
impl VTracker {
    #[new]
    fn new(versions: &Vec<String>, str_na: Option<&String>) -> Self {
        Self {
            rs: RustVTracker::new(versions, str_na),
        }
    }
}


#[pymodule]
fn vtracker(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<VTracker>()?;
    Ok(())
}
