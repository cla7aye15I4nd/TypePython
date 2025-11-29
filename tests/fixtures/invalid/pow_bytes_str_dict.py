# Cannot call pow() with Bytes, Str, Dict[str, int]
x = pow(b"hello", "hello", {"a": 1})
