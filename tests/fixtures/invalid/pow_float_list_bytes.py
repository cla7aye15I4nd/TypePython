# Cannot call pow() with Float, List[int], Bytes
x = pow(1.0, [1, 2, 3], b"hello")
