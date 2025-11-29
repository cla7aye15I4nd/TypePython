# Cannot call pow() with Bytes, None, Dict[str, int]
x = pow(b"hello", None, {"a": 1})
