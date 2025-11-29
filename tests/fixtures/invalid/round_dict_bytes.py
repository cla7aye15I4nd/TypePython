# Cannot call round() with Dict[str, int], Bytes
x = round({"a": 1}, b"hello")
