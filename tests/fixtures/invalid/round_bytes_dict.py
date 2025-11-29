# Cannot call round() with Bytes, Dict[str, int]
x = round(b"hello", {"a": 1})
