# Cannot call pow() with Bytes, Dict[str, int], None
x = pow(b"hello", {"a": 1}, None)
