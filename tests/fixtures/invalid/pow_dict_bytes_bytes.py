# Cannot call pow() with Dict[str, int], Bytes, Bytes
x = pow({"a": 1}, b"hello", b"hello")
