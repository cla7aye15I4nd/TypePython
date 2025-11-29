# Cannot call pow() with Set[int], Bool, Bytes
x = pow({1, 2, 3}, True, b"hello")
