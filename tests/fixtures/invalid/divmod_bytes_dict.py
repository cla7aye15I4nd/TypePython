# Cannot call divmod() with Bytes, Dict[str, int]
x = divmod(b"hello", {"a": 1})
