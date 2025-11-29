# Cannot call pow() with Bytes, List[int], Bytes
x = pow(b"hello", [1, 2, 3], b"hello")
