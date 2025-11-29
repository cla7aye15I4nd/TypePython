# Cannot call pow() with Bytes, Set[int], List[int]
x = pow(b"hello", {1, 2, 3}, [1, 2, 3])
