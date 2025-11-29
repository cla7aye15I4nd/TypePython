# Cannot call pow() with Bool, Dict[str, int], Set[int]
x = pow(True, {"a": 1}, {1, 2, 3})
