# Cannot call pow() with Bool, Set[int], Dict[str, int]
x = pow(True, {1, 2, 3}, {"a": 1})
