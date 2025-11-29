# Cannot call max() with Set[int], Bytes
x = max({1, 2, 3}, b"hello")
