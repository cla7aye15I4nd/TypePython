# Cannot call divmod() with Set[int], Bytes
x = divmod({1, 2, 3}, b"hello")
