# Cannot call round() with Bytes, Set[int]
x = round(b"hello", {1, 2, 3})
