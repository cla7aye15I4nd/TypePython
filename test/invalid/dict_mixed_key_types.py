# ERROR: Dict with mixed key types
# This should fail because dict keys must all have the same type

def test_mixed_key_types() -> int:
    d = {1: 100, "key": 200}  # Error: mixed int and str keys
    return 0
