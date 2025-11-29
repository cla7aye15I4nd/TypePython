# Cannot use bitwise NOT on Dict[str, int]
x: dict[str, int] = {"a": 1}
y: int = ~x
