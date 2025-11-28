# Test pass statement in various contexts

# Pass in a function body
def empty_func() -> None:
    pass

# Pass in if/else blocks
def conditional_pass(x: int) -> int:
    if x > 0:
        pass
    else:
        pass
    return x

# Pass in while loop
def loop_pass(n: int) -> int:
    i: int = 0
    while i < n:
        pass
        i = i + 1
    return i

# Call the functions
empty_func()
result1: int = conditional_pass(5)
result2: int = conditional_pass(-3)
result3: int = loop_pass(3)

print(result1)
print(result2)
print(result3)
print(b"pass tests passed")
