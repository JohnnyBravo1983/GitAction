import rust_dataframe_utils

def test_sum_numeric_columns():
    # Test med gyldige numeriske data
    data = [
        [1.0, 2.0, 3.0],  # Kolonne 1
        [4.0, 5.0, 0.0]   # Kolonne 2 (erstatt None med 0.0)
    ]
    result = rust_dataframe_utils.sum_numeric_columns(data)
    assert result == 15.0, f"Expected 15.0 but got {result}"

    # Test med tomt datasett
    data = []
    result = rust_dataframe_utils.sum_numeric_columns(data)
    assert result == 0.0, f"Expected 0.0 but got {result}"

    # Test med feil type data (skal feile)
    try:
        data = [["a", "b", "c"]]
        rust_dataframe_utils.sum_numeric_columns(data)
        assert False, "Expected an exception but none was raised"
    except Exception as e:
        assert isinstance(e, TypeError), f"Expected TypeError but got {type(e)}"