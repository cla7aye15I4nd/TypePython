# Cannot call divmod() with Set[int], Dict[str, int]
x = divmod({1, 2, 3}, {"a": 1})
