# Cannot call pow() with Float, Dict[str, int], Set[int]
x = pow(1.0, {"a": 1}, {1, 2, 3})
