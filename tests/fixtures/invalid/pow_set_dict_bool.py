# Cannot call pow() with Set[int], Dict[str, int], Bool
x = pow({1, 2, 3}, {"a": 1}, True)
