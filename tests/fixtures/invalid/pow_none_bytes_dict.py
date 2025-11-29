# Cannot call pow() with None, Bytes, Dict[str, int]
x = pow(None, b"hello", {"a": 1})
