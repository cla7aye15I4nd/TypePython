# Generator using tuple expression - tests Expression::Tuple in expr_contains_yield
def tuple_gen() -> int:
    x: int = 1
    y: int = 2
    t: tuple[int, int] = (x, y)
    yield t[0]
    yield t[1]

# Test the generator
for val in tuple_gen():
    print(val)
