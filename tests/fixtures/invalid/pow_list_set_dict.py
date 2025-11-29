# Cannot call pow() with List[int], Set[int], Dict[str, int]
x = pow([1, 2, 3], {1, 2, 3}, {"a": 1})
