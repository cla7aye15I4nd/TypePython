# Test iteration patterns in number algorithms

# Factorial
def factorial(n: int) -> int:
    result: int = 1
    for i in range(1, n + 1):
        result = result * i
    return result

for i in range(10):
    print(b"Factorial", i, factorial(i))

# Fibonacci
def fibonacci(n: int) -> list[int]:
    if n <= 0:
        return []
    if n == 1:
        return [0]

    result: list[int] = [0, 1]
    for _ in range(2, n):
        result.append(result[-1] + result[-2])
    return result

print(b"Fibonacci 10:", fibonacci(10))

# Prime check
def is_prime(n: int) -> bool:
    if n < 2:
        return False
    for i in range(2, n):
        if n % i == 0:
            return False
    return True

primes: list[int] = []
for i in range(50):
    if is_prime(i):
        primes.append(i)
print(b"Primes < 50:", primes)

# Sieve of Eratosthenes
def sieve(limit: int) -> list[int]:
    is_p: list[bool] = [True] * limit
    is_p[0] = False
    is_p[1] = False

    for i in range(2, limit):
        if is_p[i]:
            j: int = i * i
            while j < limit:
                is_p[j] = False
                j = j + i

    result: list[int] = []
    for i in range(limit):
        if is_p[i]:
            result.append(i)
    return result

print(b"Sieve 30:", sieve(30))

# GCD
def gcd(a: int, b: int) -> int:
    while b:
        a, b = b, a % b
    return a

print(b"GCD 48 18:", gcd(48, 18))

# LCM
def lcm(a: int, b: int) -> int:
    return a * b // gcd(a, b)

print(b"LCM 12 18:", lcm(12, 18))

# Prime factors
def prime_factors(n: int) -> list[int]:
    factors: list[int] = []
    d: int = 2
    while d * d <= n:
        while n % d == 0:
            factors.append(d)
            n = n // d
        d = d + 1
    if n > 1:
        factors.append(n)
    return factors

print(b"Prime factors 360:", prime_factors(360))

# Divisors
def divisors(n: int) -> list[int]:
    result: list[int] = []
    for i in range(1, n + 1):
        if n % i == 0:
            result.append(i)
    return result

print(b"Divisors 36:", divisors(36))

# Sum of digits
def digit_sum(n: int) -> int:
    total: int = 0
    while n > 0:
        total = total + n % 10
        n = n // 10
    return total

print(b"Digit sum 12345:", digit_sum(12345))

# Reverse number
def reverse_num(n: int) -> int:
    result: int = 0
    while n > 0:
        result = result * 10 + n % 10
        n = n // 10
    return result

print(b"Reverse 12345:", reverse_num(12345))

# Palindrome number
def is_palindrome_num(n: int) -> bool:
    return n == reverse_num(n)

print(b"Palindrome 12321:", is_palindrome_num(12321))
print(b"Palindrome 12345:", is_palindrome_num(12345))

# Armstrong number
def is_armstrong(n: int) -> bool:
    digits: list[int] = []
    temp: int = n
    while temp > 0:
        digits.append(temp % 10)
        temp = temp // 10

    power: int = len(digits)
    total: int = 0
    for d in digits:
        total = total + d ** power

    return total == n

armstrong: list[int] = []
for i in range(1000):
    if is_armstrong(i):
        armstrong.append(i)
print(b"Armstrong < 1000:", armstrong)

# Perfect number
def is_perfect(n: int) -> bool:
    if n < 2:
        return False
    total: int = 1
    for i in range(2, n):
        if n % i == 0:
            total = total + i
    return total == n

perfect: list[int] = []
for i in range(1, 1000):
    if is_perfect(i):
        perfect.append(i)
print(b"Perfect < 1000:", perfect)

# Power
def power(base: int, exp: int) -> int:
    result: int = 1
    for _ in range(exp):
        result = result * base
    return result

print(b"2^10:", power(2, 10))

# Binary representation
def to_binary(n: int) -> str:
    if n == 0:
        return "0"
    result: str = ""
    while n > 0:
        result = str(n % 2) + result
        n = n // 2
    return result

print(b"Binary 42:", to_binary(42))

# Count set bits
def count_bits(n: int) -> int:
    count: int = 0
    while n > 0:
        count = count + (n & 1)
        n = n >> 1
    return count

print(b"Bits in 42:", count_bits(42))

# Pascal's triangle
def pascal(n: int) -> list[list[int]]:
    result: list[list[int]] = []
    for i in range(n):
        row: list[int] = []
        for j in range(i + 1):
            if j == 0 or j == i:
                row.append(1)
            else:
                row.append(result[i - 1][j - 1] + result[i - 1][j])
        result.append(row)
    return result

print(b"Pascal's triangle 5:")
for row in pascal(5):
    print(row)

# Catalan number
def catalan(n: int) -> int:
    if n <= 1:
        return 1
    cat: list[int] = [0] * (n + 1)
    cat[0] = 1
    cat[1] = 1
    for i in range(2, n + 1):
        for j in range(i):
            cat[i] = cat[i] + cat[j] * cat[i - j - 1]
    return cat[n]

catalans: list[int] = []
for i in range(10):
    catalans.append(catalan(i))
print(b"Catalan numbers:", catalans)
