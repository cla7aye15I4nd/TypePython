# Cannot call pow() with Dict[str, int], Float, Bytes
x = pow({"a": 1}, 1.0, b"hello")
