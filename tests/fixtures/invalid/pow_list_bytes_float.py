# Cannot call pow() with List[int], Bytes, Float
x = pow([1, 2, 3], b"hello", 1.0)
