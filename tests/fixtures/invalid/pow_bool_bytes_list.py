# Cannot call pow() with Bool, Bytes, List[int]
x = pow(True, b"hello", [1, 2, 3])
