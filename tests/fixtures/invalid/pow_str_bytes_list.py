# Cannot call pow() with Str, Bytes, List[int]
x = pow("hello", b"hello", [1, 2, 3])
