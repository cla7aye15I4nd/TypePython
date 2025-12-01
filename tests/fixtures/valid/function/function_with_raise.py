# Functions containing raise - exercises contains_yield for Raise statement
def may_raise(x: int) -> int:
    if x < 0:
        raise ValueError
    return x * 2

# Call with positive value (no exception raised)
print(may_raise(5))
