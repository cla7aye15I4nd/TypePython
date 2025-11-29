# Cannot call pow() with Set[int], Dict[str, int], Set[int]
x = pow({1, 2, 3}, {"a": 1}, {1, 2, 3})
