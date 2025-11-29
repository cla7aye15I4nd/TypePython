# Cannot call pow() with Bytes, Bool, Dict[str, int]
x = pow(b"hello", True, {"a": 1})
