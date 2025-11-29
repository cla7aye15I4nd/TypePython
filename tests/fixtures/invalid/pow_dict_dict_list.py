# Cannot call pow() with Dict[str, int], Dict[str, int], List[int]
x = pow({"a": 1}, {"a": 1}, [1, 2, 3])
