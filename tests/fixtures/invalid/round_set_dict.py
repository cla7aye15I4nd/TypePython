# Cannot call round() with Set[int], Dict[str, int]
x = round({1, 2, 3}, {"a": 1})
