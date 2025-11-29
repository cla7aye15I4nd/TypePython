# Cannot call pow() with Dict[str, int], Bytes, Set[int]
x = pow({"a": 1}, b"hello", {1, 2, 3})
