# Number theory algorithms
def sum_divisors(n: int) -> int:
    sum: int = 0
    i: int = 1

    while i <= n:
        if n % i == 0:
            sum = sum + i
        i = i + 1

    return sum

def is_perfect_number(n: int) -> bool:
    divisor_sum: int = sum_divisors(n) - n
    return divisor_sum == n

def count_divisors(n: int) -> int:
    count: int = 0
    i: int = 1

    while i <= n:
        if n % i == 0:
            count = count + 1
        i = i + 1

    return count

def digital_root(n: int) -> int:
    while n >= 10:
        sum: int = 0
        temp: int = n
        while temp > 0:
            sum = sum + (temp % 10)
            temp = temp // 10
        n = sum
    return n

result1: int = sum_divisors(28)
result2: bool = is_perfect_number(28)
result3: int = count_divisors(24)
result4: int = digital_root(9875)

print(b"Sum divisors of 28:", result1)
print(b"28 is perfect:", result2)
print(b"Divisor count of 24:", result3)
print(b"Digital root of 9875:", result4)
