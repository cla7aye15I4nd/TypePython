# Cannot call pow() with Bytes, None, Set[int]
x = pow(b"hello", None, {1, 2, 3})
