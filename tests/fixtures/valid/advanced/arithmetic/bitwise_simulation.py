# Bitwise operations simulated with arithmetic
def is_power_of_two(n: int) -> bool:
    if n <= 0:
        return False
    # Check if only one bit is set
    count: int = 0
    temp: int = n
    while temp > 0:
        if temp % 2 == 1:
            count = count + 1
        temp = temp // 2
    return count == 1

def count_set_bits(n: int) -> int:
    count: int = 0
    while n > 0:
        if n % 2 == 1:
            count = count + 1
        n = n // 2
    return count

def reverse_bits_count(n: int) -> int:
    # Count bits in reverse order value
    result: int = 0
    i: int = 0
    while i < 8 and n > 0:
        result = result * 2 + (n % 2)
        n = n // 2
        i = i + 1
    return result

result1: bool = is_power_of_two(16)
result2: bool = is_power_of_two(15)
result3: int = count_set_bits(15)
result4: int = reverse_bits_count(12)

print(b"16 is power of 2:", result1)
print(b"15 is power of 2:", result2)
print(b"Set bits in 15:", result3)
print(b"Reverse bits of 12:", result4)
