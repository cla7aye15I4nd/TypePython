# Cannot call pow() with Bytes, Float, Dict[str, int]
x = pow(b"hello", 1.0, {"a": 1})
