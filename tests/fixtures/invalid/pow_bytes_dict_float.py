# Cannot call pow() with Bytes, Dict[str, int], Float
x = pow(b"hello", {"a": 1}, 1.0)
