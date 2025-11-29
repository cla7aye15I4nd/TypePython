# Cannot call pow() with Dict[str, int], List[int], Float
x = pow({"a": 1}, [1, 2, 3], 1.0)
