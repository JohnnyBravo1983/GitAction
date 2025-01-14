use polars::prelude::*;

/// Funksjon som summerer tall i alle numeriske kolonner i en DataFrame
/// 
/// # Parametre:
/// - `df`: Referanse til en Polars DataFrame.
/// 
/// # Returnerer:
/// - Summen av alle tallene i alle numeriske kolonner i DataFrame som en f64.
pub fn sum_numeric_columns(df: &DataFrame) -> f64 {
    let mut total_sum = 0.0; // Variabel for å lagre total summen

    // Iterer gjennom alle kolonnene i DataFrame
    for column in df.get_columns() {
        // Sjekk om kolonnen kan konverteres til en serie av f64-verdier
        if let Ok(series) = column.f64() {
            // Summer alle ikke-null verdier og legg til total summen
            total_sum += series.into_iter().flatten().sum::<f64>();
        }
    }

    total_sum // Returner total summen
}

#[cfg(test)]
mod tests {
    use super::*; // Importer funksjonen fra parent-modulen

    #[test]
    fn test_sum_numeric_columns() {
        // Opprett en DataFrame med både numeriske og ikke-numeriske kolonner
        let df = DataFrame::new(vec![
            Series::new("col1".into(), &[Some(1.0), Some(2.0), None] as &[Option<f64>]).into(),
            Series::new("col2".into(), &[Some(3.0), None, Some(4.0)] as &[Option<f64>]).into(),
            Series::new("col3".into(), &["a", "b", "c"] as &[&str]).into(), // Ikke-numerisk kolonne
        ])
        .unwrap();

        // Kjør funksjonen for å beregne summen
        let result = sum_numeric_columns(&df);

        // Print resultatet for visuell bekreftelse
        println!("Summen av numeriske kolonner er: {}", result);

        // Sjekk at resultatet stemmer
        assert_eq!(result, 10.0);
    }

    #[test]
    fn test_empty_dataframe() {
        // Opprett en tom DataFrame
        let df = DataFrame::new(vec![]).unwrap();

        // Kjør funksjonen for å beregne summen
        let result = sum_numeric_columns(&df);

        // Print resultatet for en tom DataFrame
        println!("Summen av tom DataFrame er: {}", result);

        // Sjekk at summen er 0 for en tom DataFrame
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_no_numeric_columns() {
        // Opprett en DataFrame uten numeriske kolonner
        let df = DataFrame::new(vec![
            Series::new("col1".into(), &["a", "b", "c"] as &[&str]).into(),
            Series::new("col2".into(), &["d", "e", "f"] as &[&str]).into(),
        ])
        .unwrap();

        // Kjør funksjonen for å beregne summen
        let result = sum_numeric_columns(&df);

        // Print resultatet for DataFrame uten numeriske kolonner
        println!("Summen av DataFrame uten numeriske kolonner er: {}", result);

        // Sjekk at summen er 0 for DataFrame uten numeriske kolonner
        assert_eq!(result, 0.0);
    }
}
