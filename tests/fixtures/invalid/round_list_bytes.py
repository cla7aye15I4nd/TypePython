# Cannot call round() with List[int], Bytes
x = round([1, 2, 3], b"hello")
