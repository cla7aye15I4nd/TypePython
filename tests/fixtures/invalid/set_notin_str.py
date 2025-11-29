# Cannot use 'not in' with Set[int] and Str
x: bool = {1, 2, 3} not in "hello"
