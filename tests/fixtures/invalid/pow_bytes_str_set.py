# Cannot call pow() with Bytes, Str, Set[int]
x = pow(b"hello", "hello", {1, 2, 3})
