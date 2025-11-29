# Cannot call pow() with Bytes, Str, List[int]
x = pow(b"hello", "hello", [1, 2, 3])
