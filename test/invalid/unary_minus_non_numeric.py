# Error: Unary minus requires numeric operand
def test() -> int:
    s: str = "hello"
    result: int = -s  # Error: unary minus on string
    return result

test()
