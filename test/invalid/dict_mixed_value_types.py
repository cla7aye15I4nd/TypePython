# ERROR: Dict with mixed value types
# This should fail because dict values must all have the same type

def test_mixed_value_types() -> int:
    d = {1: "string", 2: 42}  # Error: mixed str and int values
    return 0
