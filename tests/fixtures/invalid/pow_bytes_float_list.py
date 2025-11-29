# Cannot call pow() with Bytes, Float, List[int]
x = pow(b"hello", 1.0, [1, 2, 3])
