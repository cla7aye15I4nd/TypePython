# Cannot call pow() with Dict[str, int], Str, Set[int]
x = pow({"a": 1}, "hello", {1, 2, 3})
