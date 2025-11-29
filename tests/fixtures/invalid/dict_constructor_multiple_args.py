# dict() should not accept multiple arguments
# Expected error: "dict() takes no arguments"

d: dict[int, int] = dict(1, 2, 3)
