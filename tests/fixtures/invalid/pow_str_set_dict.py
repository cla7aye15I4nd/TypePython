# Cannot call pow() with Str, Set[int], Dict[str, int]
x = pow("hello", {1, 2, 3}, {"a": 1})
