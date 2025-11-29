# Cannot call pow() with List[int], Bytes, None
x = pow([1, 2, 3], b"hello", None)
