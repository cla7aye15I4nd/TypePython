# Cannot call pow() with Dict[str, int], List[int], Bytes
x = pow({"a": 1}, [1, 2, 3], b"hello")
