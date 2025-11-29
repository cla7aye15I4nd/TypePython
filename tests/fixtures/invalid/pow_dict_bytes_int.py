# Cannot call pow() with Dict[str, int], Bytes, Int
x = pow({"a": 1}, b"hello", 1)
