# Cannot call pow() with Float, Bytes, List[int]
x = pow(1.0, b"hello", [1, 2, 3])
