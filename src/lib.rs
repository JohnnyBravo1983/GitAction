use pyo3::prelude::*;
use polars::prelude::*;

/// Summarizes all numeric columns in a nested list.
/// 
/// # Arguments
/// 
/// - `data`: A nested list of floats (e.g., `[[1.0, 2.0], [3.0, 4.0]]`).
/// 
/// # Returns
/// 
/// The sum of all numeric values in all columns.
#[pyfunction]
pub fn sum_numeric_columns(data: Vec<Vec<f64>>) -> PyResult<f64> {
    // Convert nested list to Polars DataFrame
    let series_columns: Vec<Series> = data
        .into_iter()
        .enumerate()
        .map(|(i, col)| Series::new(format!("col_{}", i).as_str(), col))
        .collect();

    // Create DataFrame from series
    let dataframe = DataFrame::new(series_columns)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;

    // Summarize all numeric columns
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