# Cannot call pow() with Int, Bytes, Dict[str, int]
x = pow(1, b"hello", {"a": 1})
