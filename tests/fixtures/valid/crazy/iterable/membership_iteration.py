# Test membership testing with iteration

# in operator with list
nums: list[int] = [1, 2, 3, 4, 5]
print(b"3 in nums:", 3 in nums)
print(b"10 in nums:", 10 in nums)
print(b"3 not in nums:", 3 not in nums)
print(b"10 not in nums:", 10 not in nums)

# in operator with string
text: str = "hello world"
print(b"'o' in text:", "o" in text)
print(b"'x' in text:", "x" in text)
print(b"'hello' in text:", "hello" in text)
print(b"'world' in text:", "world" in text)

# in operator with set
colors: set[str] = {"red", "green", "blue"}
print(b"'red' in colors:", "red" in colors)
print(b"'yellow' in colors:", "yellow" in colors)

# in operator with dict (checks keys)
ages: dict[str, int] = {"alice": 30, "bob": 25}
print(b"'alice' in ages:", "alice" in ages)
print(b"'charlie' in ages:", "charlie" in ages)

# in operator with range
r: range = range(0, 100, 5)
print(b"25 in range:", 25 in r)
print(b"27 in range:", 27 in r)

# Membership in iteration
targets: set[int] = {2, 4, 6, 8, 10}
for n in range(1, 11):
    if n in targets:
        print(b"Found target:", n)

# Find common elements
list1: list[int] = [1, 2, 3, 4, 5]
list2: list[int] = [4, 5, 6, 7, 8]
common: list[int] = []
for n in list1:
    if n in list2:
        common.append(n)
print(b"Common:", common)

# Check substring in iteration
words: list[str] = ["hello", "world", "python", "programming"]
search: str = "o"
containing: list[str] = []
for w in words:
    if search in w:
        containing.append(w)
print(b"Words with 'o':", containing)

# Membership with bytes
data: bytes = b"hello"
print(b"104 in data:", 104 in data)  # 'h' = 104
print(b"120 in data:", 120 in data)  # 'x' = 120

# Tuple membership
coords: tuple[int, int, int] = (10, 20, 30)
print(b"20 in coords:", 20 in coords)
print(b"25 in coords:", 25 in coords)
