# Cannot call pow() with Dict[str, int], Bytes, List[int]
x = pow({"a": 1}, b"hello", [1, 2, 3])
