# Cannot call pow() with Dict[str, int], Bool, Bytes
x = pow({"a": 1}, True, b"hello")
