# Test zip() builtin

# Basic zip of two lists
names: list[str] = ["alice", "bob", "charlie"]
ages: list[int] = [30, 25, 35]
for name, age in zip(names, ages):
    print(b"Name:", name, b"Age:", age)

# Zip three lists
first: list[int] = [1, 2, 3]
second: list[int] = [10, 20, 30]
third: list[int] = [100, 200, 300]
for a, b, c in zip(first, second, third):
    print(b"Triple:", a, b, c)

# Zip with unequal lengths (stops at shortest)
short: list[int] = [1, 2]
long: list[int] = [10, 20, 30, 40]
for s, l in zip(short, long):
    print(b"Pair:", s, l)

# Zip strings
for c1, c2 in zip("abc", "xyz"):
    print(b"Chars:", c1, c2)

# Build dict from zipped lists
keys: list[str] = ["a", "b", "c"]
vals: list[int] = [1, 2, 3]
result: dict[str, int] = {}
for k, v in zip(keys, vals):
    result[k] = v
print(b"Dict:", result)
