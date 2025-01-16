import rust_dataframe_utils

# Test for å summere tallene i numeriske kolonner
def test_sum_numeric_columns():
    # Inputdata: to kolonner med tall
    data = [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]]
    result = rust_dataframe_utils.sum_numeric_columns(data)
    
    # Forventet resultat er summen av alle tallene
    assert result == 21.0, f"Expected 21.0, but got {result}"