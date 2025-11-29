# Cannot call divmod() with Dict[str, int], Bytes
x = divmod({"a": 1}, b"hello")
