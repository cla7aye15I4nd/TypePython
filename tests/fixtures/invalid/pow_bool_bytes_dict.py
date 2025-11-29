# Cannot call pow() with Bool, Bytes, Dict[str, int]
x = pow(True, b"hello", {"a": 1})
