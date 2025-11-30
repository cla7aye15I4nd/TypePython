# Test advanced comprehension patterns

# Comprehension with function call in condition
def is_prime(n: int) -> bool:
    if n < 2:
        return False
    for i in range(2, n):
        if n % i == 0:
            return False
    return True

primes: list[int] = [x for x in range(20) if is_prime(x)]
print(b"Primes:", primes)

# Comprehension with complex expression
cubes: list[int] = [x * x * x for x in range(5)]
print(b"Cubes:", cubes)

# Conditional expression in comprehension
labels: list[str] = ["even" if x % 2 == 0 else "odd" for x in range(5)]
print(b"Labels:", labels)

# Multiple conditions
filtered: list[int] = [x for x in range(50) if x % 2 == 0 if x % 3 == 0 if x % 5 == 0]
print(b"Div by 2,3,5:", filtered)

# Comprehension with string methods
words: list[str] = ["  hello  ", "  world  ", "  python  "]
stripped: list[str] = [w.strip() for w in words]
print(b"Stripped:", stripped)

uppers: list[str] = [w.strip().upper() for w in words]
print(b"Upper stripped:", uppers)

# Comprehension building dict from computation
factorials: dict[int, int] = {}
fact: int = 1
for i in range(1, 6):
    fact = fact * i
    factorials[i] = fact
print(b"Factorials:", factorials)

# Dict comprehension with value transformation
scores: dict[str, int] = {"alice": 85, "bob": 92, "charlie": 78}
grades: dict[str, str] = {name: ("A" if score >= 90 else "B" if score >= 80 else "C") for name, score in scores.items()}
print(b"Grades:", grades)

# Set comprehension removing duplicates from computation
mods: set[int] = {x % 7 for x in range(100)}
print(b"Unique mods:", mods)

# Nested comprehension for matrix transpose
matrix: list[list[int]] = [[1, 2, 3], [4, 5, 6], [7, 8, 9]]
transposed: list[list[int]] = [[row[i] for row in matrix] for i in range(3)]
print(b"Transposed:", transposed)

# Comprehension with zip
names: list[str] = ["alice", "bob", "charlie"]
ages: list[int] = [30, 25, 35]
name_age: dict[str, int] = {n: a for n, a in zip(names, ages)}
print(b"Name-age dict:", name_age)

# Comprehension with enumerate
indexed: dict[int, str] = {i: v for i, v in enumerate(["a", "b", "c"])}
print(b"Indexed:", indexed)
