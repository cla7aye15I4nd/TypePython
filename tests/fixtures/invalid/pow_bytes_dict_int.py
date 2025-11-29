# Cannot call pow() with Bytes, Dict[str, int], Int
x = pow(b"hello", {"a": 1}, 1)
