# Cannot call pow() with Set[int], Bytes, Int
x = pow({1, 2, 3}, b"hello", 1)
