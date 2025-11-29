# Cannot call round() with Set[int], Bytes
x = round({1, 2, 3}, b"hello")
