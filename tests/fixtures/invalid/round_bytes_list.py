# Cannot call round() with Bytes, List[int]
x = round(b"hello", [1, 2, 3])
