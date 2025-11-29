# Cannot call pow() with Bytes, Bytes, Dict[str, int]
x = pow(b"hello", b"hello", {"a": 1})
