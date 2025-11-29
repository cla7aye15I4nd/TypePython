# Cannot call pow() with Bytes, Int, Dict[str, int]
x = pow(b"hello", 1, {"a": 1})
