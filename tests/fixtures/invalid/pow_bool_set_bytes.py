# Cannot call pow() with Bool, Set[int], Bytes
x = pow(True, {1, 2, 3}, b"hello")
