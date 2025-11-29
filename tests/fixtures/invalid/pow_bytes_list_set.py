# Cannot call pow() with Bytes, List[int], Set[int]
x = pow(b"hello", [1, 2, 3], {1, 2, 3})
