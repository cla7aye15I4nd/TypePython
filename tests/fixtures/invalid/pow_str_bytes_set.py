# Cannot call pow() with Str, Bytes, Set[int]
x = pow("hello", b"hello", {1, 2, 3})
