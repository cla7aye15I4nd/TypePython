# Cannot call pow() with Str, Set[int], Bytes
x = pow("hello", {1, 2, 3}, b"hello")
