# Cannot call pow() with Set[int], Bytes, Bytes
x = pow({1, 2, 3}, b"hello", b"hello")
