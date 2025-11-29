# Cannot call pow() with Int, Set[int], Bytes
x = pow(1, {1, 2, 3}, b"hello")
