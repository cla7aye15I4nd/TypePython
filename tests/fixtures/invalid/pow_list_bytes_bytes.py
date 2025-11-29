# Cannot call pow() with List[int], Bytes, Bytes
x = pow([1, 2, 3], b"hello", b"hello")
