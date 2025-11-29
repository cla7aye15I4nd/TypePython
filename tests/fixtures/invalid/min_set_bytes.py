# Cannot call min() with Set[int], Bytes
x = min({1, 2, 3}, b"hello")
