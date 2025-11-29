# Cannot call pow() with Bytes, List[int], Str
x = pow(b"hello", [1, 2, 3], "hello")
