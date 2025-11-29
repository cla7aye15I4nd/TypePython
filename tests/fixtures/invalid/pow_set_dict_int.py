# Cannot call pow() with Set[int], Dict[str, int], Int
x = pow({1, 2, 3}, {"a": 1}, 1)
