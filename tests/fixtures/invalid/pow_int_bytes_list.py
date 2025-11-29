# Cannot call pow() with Int, Bytes, List[int]
x = pow(1, b"hello", [1, 2, 3])
