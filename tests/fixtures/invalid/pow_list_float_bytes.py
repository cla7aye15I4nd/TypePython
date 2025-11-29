# Cannot call pow() with List[int], Float, Bytes
x = pow([1, 2, 3], 1.0, b"hello")
