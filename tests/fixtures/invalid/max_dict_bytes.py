# Cannot call max() with Dict[str, int], Bytes
x = max({"a": 1}, b"hello")
