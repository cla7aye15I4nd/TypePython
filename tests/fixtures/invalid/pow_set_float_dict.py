# Cannot call pow() with Set[int], Float, Dict[str, int]
x = pow({1, 2, 3}, 1.0, {"a": 1})
