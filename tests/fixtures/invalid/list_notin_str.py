# Cannot use 'not in' with List[int] and Str
x: bool = [1, 2, 3] not in "hello"
