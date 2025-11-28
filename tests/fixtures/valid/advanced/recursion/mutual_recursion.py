# Mutual recursion (even/odd check)
def is_even(n: int) -> bool:
    if n == 0:
        return True
    else:
        return is_odd(n - 1)

def is_odd(n: int) -> bool:
    if n == 0:
        return False
    else:
        return is_even(n - 1)

def count_even_odd(limit: int) -> int:
    even_count: int = 0
    odd_count: int = 0
    i: int = 0

    while i <= limit:
        if is_even(i):
            even_count = even_count + 1
        else:
            odd_count = odd_count + 1
        i = i + 1

    return even_count * 100 + odd_count

result: int = count_even_odd(10)
print("Even/Odd count result:", result)

print("Is 0 even:", is_even(0))
print("Is 1 even:", is_even(1))
print("Is 10 even:", is_even(10))
print("Is 15 even:", is_even(15))
