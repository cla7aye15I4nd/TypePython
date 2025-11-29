# Cannot call pow() with Set[int], Dict[str, int], Bytes
x = pow({1, 2, 3}, {"a": 1}, b"hello")
