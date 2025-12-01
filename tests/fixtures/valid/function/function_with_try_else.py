# Function with try/except/else block - exercises else_block in contains_yield
def try_else_fn(x: int) -> int:
    result: int = 0
    try:
        result = x * 2
    except:
        result = -1
    else:
        result = result + 10
    return result

print(try_else_fn(5))
