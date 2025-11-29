# Cannot call pow() with Set[int], Bytes, List[int]
x = pow({1, 2, 3}, b"hello", [1, 2, 3])
