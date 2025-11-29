# Cannot call min() with Dict[str, int], Bytes
x = min({"a": 1}, b"hello")
