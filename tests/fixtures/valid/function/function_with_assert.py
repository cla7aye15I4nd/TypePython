# Functions containing assert - exercises contains_yield for Assert statement
def checked_add(a: int, b: int) -> int:
    assert a > 0
    assert b > 0
    return a + b

print(checked_add(5, 3))

def assert_with_msg(x: int) -> int:
    assert x > 0, "x must be positive"
    return x * 2

print(assert_with_msg(10))
