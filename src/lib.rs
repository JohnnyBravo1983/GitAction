use pyo3::prelude::*;
use polars::prelude::*;

/// Summerer tallene i alle numeriske kolonner fra en nested list
#[pyfunction]
pub fn sum_numeric_columns(data: Vec<Vec<f64>>) -> PyResult<f64> {
    // Konverter nested list til Polars DataFrame
    let series_columns: Vec<Series> = data
        .into_iter()
        .enumerate()
        .map(|(i, col)| Series::new(format!("col_{}", i).into(), col))
        .collect();

    // Konverter Series til Columns
    let mut columns = Vec::new();
    for series in series_columns {
        columns.push(series.into_column());
    }

    // Opprett DataFrame
    let dataframe = DataFrame::new(columns).map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;

    // Summer alle numeriske kolonner
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


// Dette er en testendring for å trigge en GitHub Actions-workflow