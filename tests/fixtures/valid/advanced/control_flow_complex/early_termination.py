# Loop patterns with early termination conditions
def find_first_divisor(n: int, start: int) -> int:
    divisor: int = start
    found: int = 0

    while divisor < n:
        if n % divisor == 0:
            found = divisor
            divisor = n
        divisor = divisor + 1

    return found

def search_pattern(limit: int, target: int) -> int:
    i: int = 1
    result: int = -1

    while i <= limit:
        if i * i == target:
            result = i
            i = limit + 1
        else:
            i = i + 1

    return result

def collatz_steps(n: int) -> int:
    steps: int = 0
    current: int = n

    while current != 1:
        if current % 2 == 0:
            current = current // 2
        else:
            current = current * 3 + 1
        steps = steps + 1

        if steps > 1000:
            current = 1

    return steps

result1: int = find_first_divisor(100, 2)
result2: int = search_pattern(20, 144)
result3: int = collatz_steps(27)

print("First divisor of 100:", result1)
print("Square root search:", result2)
print("Collatz steps for 27:", result3)
