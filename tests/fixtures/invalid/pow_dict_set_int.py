# Cannot call pow() with Dict[str, int], Set[int], Int
x = pow({"a": 1}, {1, 2, 3}, 1)
