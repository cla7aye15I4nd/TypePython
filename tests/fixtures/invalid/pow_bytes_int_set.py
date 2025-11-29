# Cannot call pow() with Bytes, Int, Set[int]
x = pow(b"hello", 1, {1, 2, 3})
