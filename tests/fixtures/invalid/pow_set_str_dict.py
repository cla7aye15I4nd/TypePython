# Cannot call pow() with Set[int], Str, Dict[str, int]
x = pow({1, 2, 3}, "hello", {"a": 1})
