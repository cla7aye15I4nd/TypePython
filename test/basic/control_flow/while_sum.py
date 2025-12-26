# Sum numbers using while loop
def sum_to_n(n: int) -> int:
    result: int = 0
    i: int = 1
    while i <= n:
        result = result + i
        i += 1
    return result
