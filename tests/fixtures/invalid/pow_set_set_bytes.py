# Cannot call pow() with Set[int], Set[int], Bytes
x = pow({1, 2, 3}, {1, 2, 3}, b"hello")
