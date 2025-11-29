# Cannot call pow() with List[int], None, Bytes
x = pow([1, 2, 3], None, b"hello")
