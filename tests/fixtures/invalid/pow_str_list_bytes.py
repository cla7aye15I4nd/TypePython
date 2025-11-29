# Cannot call pow() with Str, List[int], Bytes
x = pow("hello", [1, 2, 3], b"hello")
