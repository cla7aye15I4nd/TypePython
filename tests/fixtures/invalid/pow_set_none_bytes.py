# Cannot call pow() with Set[int], None, Bytes
x = pow({1, 2, 3}, None, b"hello")
