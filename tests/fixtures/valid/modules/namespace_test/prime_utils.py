def is_prime(n: int) -> bool:
    if n < 2:
        return False
    i: int = 2
    while i * i <= n:
        if n % i == 0:
            return False
        i = i + 1
    return True

def next_prime(n: int) -> int:
    candidate: int = n + 1
    while not is_prime(candidate):
        candidate = candidate + 1
    return candidate

def count_primes(limit: int) -> int:
    count: int = 0
    i: int = 2
    while i <= limit:
        if is_prime(i):
            count = count + 1
        i = i + 1
    return count
