# Cannot call pow() with Bytes, Dict[str, int], Dict[str, int]
x = pow(b"hello", {"a": 1}, {"a": 1})
