# Cannot call pow() with Dict[str, int], Int, Set[int]
x = pow({"a": 1}, 1, {1, 2, 3})
