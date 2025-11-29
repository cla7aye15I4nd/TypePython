# Cannot call pow() with Bytes, Bool, List[int]
x = pow(b"hello", True, [1, 2, 3])
