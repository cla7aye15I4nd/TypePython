# Cannot call divmod() with Bytes, Set[int]
x = divmod(b"hello", {1, 2, 3})
