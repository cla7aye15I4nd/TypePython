# Cannot call pow() with Bytes, Set[int], Int
x = pow(b"hello", {1, 2, 3}, 1)
