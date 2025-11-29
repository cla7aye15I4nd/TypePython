# Cannot call pow() with Dict[str, int], List[int], Dict[str, int]
x = pow({"a": 1}, [1, 2, 3], {"a": 1})
