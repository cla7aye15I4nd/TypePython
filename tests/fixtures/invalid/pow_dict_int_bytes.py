# Cannot call pow() with Dict[str, int], Int, Bytes
x = pow({"a": 1}, 1, b"hello")
