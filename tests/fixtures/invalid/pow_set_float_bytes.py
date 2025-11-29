# Cannot call pow() with Set[int], Float, Bytes
x = pow({1, 2, 3}, 1.0, b"hello")
