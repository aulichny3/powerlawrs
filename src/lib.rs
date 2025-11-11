use powerlaw::dist::pareto;
use powerlaw::dist::pareto::gof::Fitment;
use powerlaw::dist::pareto::hypothesis::H0;
use pyo3::prelude::*;

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

    Ok(())
}
/*
// 1. Define a wrapper struct with #[pyclass]
#[pyclass]
struct PyFitment {
    // Add fields that match the ones you need from the original Fitment struct
    #[pyo3(get, set)]
    x_min: f64,
    #[pyo3(get, set)]
    alpha: f64,
    #[pyo3(get, set)]
    D: f64,
    #[pyo3(get, set)]
    len_tail: usize,
}

#[pyclass]
struct PyH0 {
    #[pyo3(get, set)]
    pub gt: usize,
    #[pyo3(get, set)]
    pub total: usize,
    #[pyo3(get, set)]
    pub pval: f64,
}

#[pyfunction]
fn hypothesis_test(data: Vec<f64>, prec: f64, alpha: f64, x_min: f64, best_d: f64) -> PyResult<PyH0>  {
    let H0 = pareto::hypothesis_test(data, prec, alpha, x_min, best_d);

    let py_result =  PyH0::from_rust_H0fitment(H0);

    Ok(py_result)

}


#[pyfunction]
fn gof(data: Vec<f64>, x_mins: Vec<f64>, alphas: Vec<f64>) -> PyResult<PyFitment>  {
    let result = pareto::gof(&data, &x_mins, &alphas);

    let py_result = PyFitment::from_rust_fitment(result);
    Ok(py_result)
}

#[pyfunction]
fn find_alphas_fast(mut data: Vec<f64>) -> PyResult<(Vec<f64>, Vec<f64>)>  {
    let result = pareto::find_alphas_fast(data.as_mut_slice());
    Ok(result)
}


/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn powerlawrs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(find_alphas_fast, m)?)?;
    m.add_function(wrap_pyfunction!(gof, m)?)?;
    m.add_function(wrap_pyfunction!(hypothesis_test, m)?)?;
    m.add_function(wrap_pyfunction!(mean, m)?)?;
    Ok(())
}


impl PyFitment {
    // You might want a method to convert from the original Rust type
    pub fn from_rust_fitment(fitment: Fitment) -> Self {
        PyFitment {
            // Assuming Fitment has these fields (adjust as needed)
            x_min: fitment.x_min,
            alpha: fitment.alpha,
            D: fitment.D,
            len_tail: fitment.len_tail
            // ...
        }
    }
}

impl PyH0 {
    pub fn from_rust_H0fitment(pyh0: H0) -> Self {
        PyH0 {
            // Assuming Fitment has these fields (adjust as needed)
            gt: pyh0.gt,
            total: pyh0.total,
            pval: pyh0.pval
            // ...
        }
    }
}

*/
