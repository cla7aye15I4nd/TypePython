# Cannot call pow() with Bytes, Set[int], Bool
x = pow(b"hello", {1, 2, 3}, True)
