# Cannot call pow() with Dict[str, int], Str, Bytes
x = pow({"a": 1}, "hello", b"hello")
