# Cannot call pow() with Dict[str, int], Bytes, Str
x = pow({"a": 1}, b"hello", "hello")
