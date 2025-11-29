# Cannot call pow() with Bytes, Dict[str, int], Bool
x = pow(b"hello", {"a": 1}, True)
