# Cannot call pow() with Bytes, Dict[str, int], Bytes
x = pow(b"hello", {"a": 1}, b"hello")
