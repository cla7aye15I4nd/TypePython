# Cannot use 'in' with Set[int] and Str
x: bool = {1, 2, 3} in "hello"
