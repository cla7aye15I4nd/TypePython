# Cannot call pow() with Set[int], Dict[str, int], Dict[str, int]
x = pow({1, 2, 3}, {"a": 1}, {"a": 1})
