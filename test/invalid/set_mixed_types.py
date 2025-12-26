# ERROR: Set with mixed element types
# This should fail because set elements must all have the same type

def test_mixed_set_types() -> int:
    s = {1, 2, "string", 4}  # Error: mixed int and str elements
    return 0
