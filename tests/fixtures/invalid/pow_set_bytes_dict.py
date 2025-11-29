# Cannot call pow() with Set[int], Bytes, Dict[str, int]
x = pow({1, 2, 3}, b"hello", {"a": 1})
