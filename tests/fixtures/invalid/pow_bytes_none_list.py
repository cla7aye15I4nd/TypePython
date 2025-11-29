# Cannot call pow() with Bytes, None, List[int]
x = pow(b"hello", None, [1, 2, 3])
