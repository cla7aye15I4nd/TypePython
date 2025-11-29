# Cannot call pow() with None, Set[int], Bytes
x = pow(None, {1, 2, 3}, b"hello")
