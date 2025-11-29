# Cannot call pow() with Dict[str, int], None, Set[int]
x = pow({"a": 1}, None, {1, 2, 3})
