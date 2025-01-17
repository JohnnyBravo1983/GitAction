use pyo3::prelude::*;
use polars::prelude::*;

/// Summerer tallene i alle numeriske kolonner fra en nested list
#[pyfunction]
pub fn sum_numeric_columns(data: Vec<Vec<f64>>) -> PyResult<f64> {
    let series_columns: Vec<Series> = data
        .into_iter()
        .enumerate()
        .map(|(i, col)| Series::new(format!("col_{}", i), col))
        .collect();

    let dataframe = DataFrame::new(series_columns).map_err(|e| {
        pyo3::exceptions::PyValueError::new_err(format!("Failed to create DataFrame: {}", e))
    })?;

    let mut total_sum = 0.0;
    for column in dataframe.get_columns() {
        if let Ok(series) = column.f64() {
            total_sum += series.into_iter().flatten().sum::<f64>();
        }
    }

    Ok(total_sum)
}

#[pymodule]
fn rust_dataframe_utils(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_numeric_columns, m)?)?;
    Ok(())
}

