# Cannot call pow() with Set[int], Str, Bytes
x = pow({1, 2, 3}, "hello", b"hello")
