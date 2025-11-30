# Test reduce-like patterns

# Manual reduce for sum
nums: list[int] = [1, 2, 3, 4, 5]
acc: int = 0
for n in nums:
    acc = acc + n
print(b"Sum:", acc)

# Manual reduce for product
product: int = 1
for n in nums:
    product = product * n
print(b"Product:", product)

# Manual reduce for max
values: list[int] = [3, 1, 4, 1, 5, 9, 2, 6]
max_val: int = values[0]
for v in values:
    if v > max_val:
        max_val = v
print(b"Max:", max_val)

# Manual reduce for min
min_val: int = values[0]
for v in values:
    if v < min_val:
        min_val = v
print(b"Min:", min_val)

# Reduce to find longest string
words: list[str] = ["cat", "elephant", "dog", "hippopotamus"]
longest: str = words[0]
for w in words:
    if len(w) > len(longest):
        longest = w
print(b"Longest:", longest)

# Reduce to concatenate
strings: list[str] = ["hello", " ", "world", "!"]
result: str = ""
for s in strings:
    result = result + s
print(b"Concatenated:", result)

# Reduce with initial value
numbers: list[int] = [1, 2, 3, 4, 5]
sum_plus_100: int = 100
for n in numbers:
    sum_plus_100 = sum_plus_100 + n
print(b"Sum + 100:", sum_plus_100)

# Factorial via reduce pattern
n: int = 5
factorial: int = 1
for i in range(1, n + 1):
    factorial = factorial * i
print(b"Factorial of 5:", factorial)

# GCD via reduce pattern
def gcd(a: int, b: int) -> int:
    while b:
        a, b = b, a % b
    return a

values2: list[int] = [48, 64, 80, 96]
result_gcd: int = values2[0]
for v in values2[1:]:
    result_gcd = gcd(result_gcd, v)
print(b"GCD:", result_gcd)

# Count occurrences via reduce
data: list[str] = ["a", "b", "a", "c", "a", "b"]
counts: dict[str, int] = {}
for item in data:
    counts[item] = counts.get(item, 0) + 1
print(b"Counts:", counts)
