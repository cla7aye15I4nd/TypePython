# Cannot call pow() with List[int], Int, Bytes
x = pow([1, 2, 3], 1, b"hello")
