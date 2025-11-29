# Cannot call pow() with Set[int], List[int], Bytes
x = pow({1, 2, 3}, [1, 2, 3], b"hello")
