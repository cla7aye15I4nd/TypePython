# Cannot call pow() with Str, Dict[str, int], Set[int]
x = pow("hello", {"a": 1}, {1, 2, 3})
