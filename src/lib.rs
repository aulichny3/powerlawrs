use pyo3::prelude::*;

pub mod dist;
pub mod stats;
pub mod util;

/// A Python module implemented in Rust.
#[pymodule]
fn powerlawrs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Add the functions/classes from the other files as submodules
    m.add_submodule(&stats::descriptive::create_module(m.py())?)?;
    m.add_submodule(&stats::random::create_module(m.py())?)?;
    m.add_submodule(&stats::ks::create_module(m.py())?)?;
    m.add_submodule(&util::create_module(m.py())?)?;
    m.add_submodule(&util::sim::create_module(m.py())?)?;
    m.add_submodule(&dist::create_module(m.py())?)?;

    Ok(())
}
