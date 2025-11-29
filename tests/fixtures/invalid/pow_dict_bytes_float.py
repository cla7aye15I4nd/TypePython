# Cannot call pow() with Dict[str, int], Bytes, Float
x = pow({"a": 1}, b"hello", 1.0)
