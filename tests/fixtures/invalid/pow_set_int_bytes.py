# Cannot call pow() with Set[int], Int, Bytes
x = pow({1, 2, 3}, 1, b"hello")
