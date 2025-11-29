# Cannot use bitwise NOT on Set[int]
x: set[int] = {1, 2, 3}
y: int = ~x
