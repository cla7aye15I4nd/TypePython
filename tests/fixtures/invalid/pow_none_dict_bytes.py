# Cannot call pow() with None, Dict[str, int], Bytes
x = pow(None, {"a": 1}, b"hello")
