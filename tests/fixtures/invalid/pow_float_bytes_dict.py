# Cannot call pow() with Float, Bytes, Dict[str, int]
x = pow(1.0, b"hello", {"a": 1})
