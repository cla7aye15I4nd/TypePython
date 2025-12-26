# Edge case tests

# Test expression statement
def expr_stmt() -> int:
    x: int = 5
    x + 3  # expression statement (ignored)
    return x

# Test nested if-else
def nested_if(x: int) -> int:
    result: int = 0
    if x > 10:
        if x > 20:
            result = 3
        else:
            result = 2
    else:
        if x > 5:
            result = 1
        else:
            result = 0
    return result

# Test while with break-like pattern (using condition)
def count_to_limit(n: int) -> int:
    count: int = 0
    i: int = 0
    while i < n:
        count += 1
        i += 1
    return count

# Test chained comparisons (converted to single comparison for now)
def in_range(x: int, low: int, high: int) -> int:
    result: int = 0
    if x >= low:
        if x <= high:
            result = 1
    return result

# Test chained comparison a < b < c
def chained_compare(a: int, b: int, c: int) -> int:
    if a < b < c:
        return 1
    return 0
