# Error: Right operand of binary operator must be numeric
def test() -> int:
    x: int = 5
    s: str = "hello"
    result: int = x + s  # Error: right operand is string
    return result

test()
