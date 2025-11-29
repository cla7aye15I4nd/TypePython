# Cannot call pow() with Int, Bytes, Set[int]
x = pow(1, b"hello", {1, 2, 3})
