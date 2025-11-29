# Cannot call pow() with Str, Bytes, Dict[str, int]
x = pow("hello", b"hello", {"a": 1})
