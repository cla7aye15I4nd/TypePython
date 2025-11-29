# Cannot call pow() with Dict[str, int], Set[int], Bool
x = pow({"a": 1}, {1, 2, 3}, True)
