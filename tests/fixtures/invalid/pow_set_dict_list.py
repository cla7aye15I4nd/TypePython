# Cannot call pow() with Set[int], Dict[str, int], List[int]
x = pow({1, 2, 3}, {"a": 1}, [1, 2, 3])
