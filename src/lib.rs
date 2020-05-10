#![allow(unused_variables)]

mod engine;
mod biquads;


fn main() {
    use pyo3::prelude::*;
    use pyo3::wrap_pyfunction;

    
    

    #[pyfunction]
    /// get audio samples from the output buffer in python
    fn pop_buffer() -> PyResult<f32> {


        Ok(0.0)
    }
    

    #[pymodule]
    /// A Python module implemented in Rust.
    fn subtractesizer(py: Python, m: &PyModule) -> PyResult<()> {
        m.add_wrapped(wrap_pyfunction!(pop_buffer))?;

        Ok(())
    }
}
