# Cannot call pow() with Dict[str, int], Set[int], List[int]
x = pow({"a": 1}, {1, 2, 3}, [1, 2, 3])
