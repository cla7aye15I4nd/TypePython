# Cannot call pow() with Str, Dict[str, int], Bytes
x = pow("hello", {"a": 1}, b"hello")
