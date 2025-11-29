# Cannot call pow() with Bytes, Dict[str, int], List[int]
x = pow(b"hello", {"a": 1}, [1, 2, 3])
