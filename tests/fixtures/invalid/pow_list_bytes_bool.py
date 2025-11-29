# Cannot call pow() with List[int], Bytes, Bool
x = pow([1, 2, 3], b"hello", True)
