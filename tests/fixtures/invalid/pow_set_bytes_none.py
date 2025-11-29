# Cannot call pow() with Set[int], Bytes, None
x = pow({1, 2, 3}, b"hello", None)
