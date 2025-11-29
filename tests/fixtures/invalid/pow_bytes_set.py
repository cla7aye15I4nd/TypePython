# Cannot call pow() with Bytes, Set[int]
x = pow(b"hello", {1, 2, 3})
