use pyo3::prelude::*;
use crossbeam; // 0.8.0

#[pyfunction]
fn parallel_sorting (mut data: Vec<isize>, threads: usize, arr_size: usize) -> PyResult<Vec<isize>>{
    let chunks = std::cmp::min(arr_size, threads);
    let _ = crossbeam::scope(|scope| {
        for slice in data.chunks_mut(arr_size / chunks) {
            scope.spawn(move |_| slice.sort());
        }
    });
    data.sort();
    Ok(data)
}

#[pymodule]
fn parallelsort_rust(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parallel_sorting, m)?)?;
    Ok(())
}