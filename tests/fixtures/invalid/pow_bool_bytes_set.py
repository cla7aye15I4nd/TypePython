# Cannot call pow() with Bool, Bytes, Set[int]
x = pow(True, b"hello", {1, 2, 3})
