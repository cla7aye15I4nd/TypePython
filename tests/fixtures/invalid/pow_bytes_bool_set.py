# Cannot call pow() with Bytes, Bool, Set[int]
x = pow(b"hello", True, {1, 2, 3})
