# Cannot use 'not in' with Set[int] and None
x: bool = {1, 2, 3} not in None
