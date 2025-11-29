# Cannot call pow() with Bytes, Set[int], Float
x = pow(b"hello", {1, 2, 3}, 1.0)
