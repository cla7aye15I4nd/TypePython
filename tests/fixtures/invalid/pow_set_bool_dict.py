# Cannot call pow() with Set[int], Bool, Dict[str, int]
x = pow({1, 2, 3}, True, {"a": 1})
