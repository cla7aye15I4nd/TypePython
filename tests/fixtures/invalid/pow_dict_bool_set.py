# Cannot call pow() with Dict[str, int], Bool, Set[int]
x = pow({"a": 1}, True, {1, 2, 3})
