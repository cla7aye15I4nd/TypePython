# Cannot call pow() with None, Bytes, Set[int]
x = pow(None, b"hello", {1, 2, 3})
