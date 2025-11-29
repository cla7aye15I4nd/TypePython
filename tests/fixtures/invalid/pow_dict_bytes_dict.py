# Cannot call pow() with Dict[str, int], Bytes, Dict[str, int]
x = pow({"a": 1}, b"hello", {"a": 1})
