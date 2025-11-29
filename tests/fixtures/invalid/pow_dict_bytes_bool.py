# Cannot call pow() with Dict[str, int], Bytes, Bool
x = pow({"a": 1}, b"hello", True)
