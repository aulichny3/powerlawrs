use pyo3::prelude::*;

pub mod descriptive {
    use powerlaw::stats;
    use pyo3::prelude::*;

    #[pyfunction]
    fn mean(data: Vec<f64>) -> PyResult<f64> {
        let mu = stats::descriptive::mean(&data);
        Ok(mu)
    }

    #[pyfunction]
    fn variance(data: Vec<f64>, ddof: u8) -> PyResult<f64> {
        let sigma2 = stats::descriptive::variance(&data, ddof);
        Ok(sigma2)
    }

    /// Creates the 'mean' submodule
    pub fn create_module(py: Python<'_>) -> PyResult<Bound<'_, PyModule>> {
        // Create a new Python submodule
        let m = PyModule::new(py, "descriptive")?;

        // Add the wrapped function to the submodule
        m.add_function(wrap_pyfunction!(mean, &m)?)?;
        m.add_function(wrap_pyfunction!(variance, &m)?)?;

        Ok(m)
    }
}

pub mod random {
    use powerlaw::stats;
    use pyo3::prelude::*;

    #[pyfunction]
    fn random_choice(data: Vec<f64>, size: usize) -> PyResult<Vec<f64>> {
        let samp = stats::random::random_choice(&data, size);
        Ok(samp)
    }

    #[pyfunction]
    fn random_uniform(n: usize) -> PyResult<Vec<f64>> {
        let u = stats::random::random_uniform(n);
        Ok(u)
    }

    /// Creates the 'mean' submodule
    pub fn create_module(py: Python<'_>) -> PyResult<Bound<'_, PyModule>> {
        // Create a new Python submodule
        let m = PyModule::new(py, "random")?;

        // Add the wrapped function to the submodule
        m.add_function(wrap_pyfunction!(random_choice, &m)?)?;
        m.add_function(wrap_pyfunction!(random_uniform, &m)?)?;

        Ok(m)
    }
}

pub mod ks {
    use pyo3::exceptions::{PyTypeError, PyValueError};
    use pyo3::prelude::*;

    // -----------------------------------------------------------------
    // 1. YOUR ORIGINAL RUST HELPER LOGIC
    // (I've added standard implementations for these) - Thank you @Google Gemini
    // -----------------------------------------------------------------

    /// The D+ statistic measures the largest amount by which the ECDF is above the theoretical CDF.
    fn compute_dplus(cdfvals: &[f64], n: usize) -> f64 {
        (1..=n)
            .map(|i| i as f64 / n as f64 - cdfvals[i - 1])
            .fold(f64::MIN, f64::max)
    }

    /// The D- statistic measures the largest amount by which the ECDF is below the theoretical CDF.
    fn compute_dminus(cdfvals: &[f64], n: usize) -> f64 {
        /*
        Computes D- as used in the Kolmogorov-Smirnov test.
        ...
        */
        (0..n)
            .map(|i| cdfvals[i] - i as f64 / n as f64)
            .fold(f64::MIN, f64::max)
    }

    // -----------------------------------------------------------------
    // 2. THE PYFUNCTION WRAPPER
    // This replaces your generic `ks_1sam_sorted` function
    // -----------------------------------------------------------------

    /// 1 sample KS test based on a known cdf.
    ///
    /// Args:
    ///     sorted_x (list[float]): A list of data points, pre-sorted in ascending order.
    ///     cdf_func (callable): A Python function (or lambda) that takes a single float
    ///         (x) and returns its cumulative probability F(x) as a float.
    ///
    /// Returns:
    ///     tuple[float, float, float]: A tuple containing (D+, D-, D).
    ///
    /// Raises:
    ///     ValueError: If the list is empty.
    ///     TypeError: If the cdf_func does not return a float.
    ///     (Any exception): Any exception raised by the cdf_func will be propagated.
    #[pyfunction]
    fn ks_1sam_sorted(
        py: Python<'_>,
        sorted_x: Vec<f64>,
        cdf_func: PyObject,
    ) -> PyResult<(f64, f64, f64)> {
        let n = sorted_x.len();
        if n == 0 {
            // Return a Python-style error, not a (0,0,0) tuple
            return Err(PyValueError::new_err(
                "Input list 'sorted_x' cannot be empty.",
            ));
        }

        // This is the logic from your original function, but now
        // it can properly handle Python errors.
        let mut cdfvals: Vec<f64> = Vec::with_capacity(n);

        for &x in sorted_x.iter() {
            // 1. Create Python arguments (a 1-element tuple)
            let args = (x,);

            // 2. Call the user's Python function
            // The '?' will propagate any exception (e.g., ValueError, ZeroDivisionError)
            let cdf_val_py = cdf_func.call1(py, args)?;

            // 3. Extract the Python float result back to a Rust f64
            // The '?' will propagate a TypeError if the user returned None, a string, etc.
            let cdf_val: f64 = cdf_val_py
                .extract(py)
                .map_err(|_| PyTypeError::new_err("cdf_func must return a float"))?;

            cdfvals.push(cdf_val);
        }

        // The rest of the logic is the same
        let dplus = compute_dplus(&cdfvals, n);
        let dminus = compute_dminus(&cdfvals, n);
        let d = dplus.max(dminus);

        Ok((dplus, dminus, d))
    }

    // -----------------------------------------------------------------
    // 3. THE MODULE CREATION FUNCTION
    // -----------------------------------------------------------------

    /// Creates the 'ks' Python submodule
    pub fn create_module(py: Python<'_>) -> PyResult<Bound<'_, PyModule>> {
        // This module will be named 'ks' in Python
        let m = PyModule::new(py, "ks")?;

        // Add the wrapper function to the module
        m.add_function(wrap_pyfunction!(ks_1sam_sorted, &m)?)?;

        Ok(m)
    }
}
