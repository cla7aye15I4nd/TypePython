# Cannot call pow() with Dict[str, int], None, Bytes
x = pow({"a": 1}, None, b"hello")
