# Cannot call pow() with Bytes, Set[int], Str
x = pow(b"hello", {1, 2, 3}, "hello")
