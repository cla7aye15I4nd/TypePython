# Cannot call pow() with Bytes, Set[int], None
x = pow(b"hello", {1, 2, 3}, None)
