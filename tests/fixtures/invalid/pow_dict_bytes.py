# Cannot call pow() with Dict[str, int], Bytes
x = pow({"a": 1}, b"hello")
