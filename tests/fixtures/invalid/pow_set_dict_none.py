# Cannot call pow() with Set[int], Dict[str, int], None
x = pow({1, 2, 3}, {"a": 1}, None)
