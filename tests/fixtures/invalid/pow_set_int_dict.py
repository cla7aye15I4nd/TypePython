# Cannot call pow() with Set[int], Int, Dict[str, int]
x = pow({1, 2, 3}, 1, {"a": 1})
