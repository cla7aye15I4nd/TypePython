# Cannot call pow() with Set[int], Bytes, Str
x = pow({1, 2, 3}, b"hello", "hello")
