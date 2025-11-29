# Cannot call min() with Bytes, Set[int]
x = min(b"hello", {1, 2, 3})
