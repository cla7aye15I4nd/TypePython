# Cannot call pow() with Dict[str, int], Set[int]
x = pow({"a": 1}, {1, 2, 3})
