# Prime number sieve (limited)
def is_prime(n: int) -> bool:
    if n < 2:
        return False
    if n == 2:
        return True
    if n % 2 == 0:
        return False

    i: int = 3
    while i * i <= n:
        if n % i == 0:
            return False
        i = i + 2

    return True

def count_primes_in_range(start: int, end: int) -> int:
    count: int = 0
    i: int = start

    while i <= end:
        if is_prime(i):
            count = count + 1
        i = i + 1

    return count

def sum_primes_under(n: int) -> int:
    sum: int = 0
    i: int = 2

    while i < n:
        if is_prime(i):
            sum = sum + i
        i = i + 1

    return sum

result1: int = count_primes_in_range(1, 50)
result2: int = sum_primes_under(20)
result3: int = count_primes_in_range(100, 120)

print("Primes 1-50:", result1)
print("Sum primes under 20:", result2)
print("Primes 100-120:", result3)
