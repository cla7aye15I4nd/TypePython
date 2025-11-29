# Cannot call pow() with Set[int], Float, Set[int]
x = pow({1, 2, 3}, 1.0, {1, 2, 3})
