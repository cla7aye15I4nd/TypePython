# Edge case: minimal functions
def do_nothing() -> None:
    x: int = 0

def return_immediately() -> int:
    return 42

def simple_pass_through(x: int) -> int:
    return x

def minimal_conditional(x: int) -> int:
    if x > 0:
        return 1
    else:
        return 0

do_nothing()
result1: int = return_immediately()
result2: int = simple_pass_through(100)
result3: int = minimal_conditional(5)
result4: int = minimal_conditional(-5)

print("Return immediately:", result1)
print("Pass through:", result2)
print("Minimal conditional positive:", result3)
print("Minimal conditional negative:", result4)
