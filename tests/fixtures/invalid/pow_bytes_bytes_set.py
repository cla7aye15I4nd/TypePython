# Cannot call pow() with Bytes, Bytes, Set[int]
x = pow(b"hello", b"hello", {1, 2, 3})
