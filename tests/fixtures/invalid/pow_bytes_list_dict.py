# Cannot call pow() with Bytes, List[int], Dict[str, int]
x = pow(b"hello", [1, 2, 3], {"a": 1})
