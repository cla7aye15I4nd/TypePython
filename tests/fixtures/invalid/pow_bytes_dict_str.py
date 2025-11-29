# Cannot call pow() with Bytes, Dict[str, int], Str
x = pow(b"hello", {"a": 1}, "hello")
