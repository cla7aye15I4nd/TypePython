# Test int membership operations (in, not in)

# Int in list
lst: list[int] = [1, 2, 3, 4, 5]
if 3 in lst:
    print(1)

if 10 not in lst:
    print(2)

# Int in set
s: set[int] = {10, 20, 30}
if 20 in s:
    print(3)

if 40 not in s:
    print(4)

# Int in dict (checks keys)
d: dict[int, int] = {1: 10, 2: 20, 3: 30}
if 2 in d:
    print(5)

if 5 not in d:
    print(6)

# Int in bytes (checks byte values)
b: bytes = b"hello"
if 104 in b:  # 'h' = 104
    print(7)

if 200 not in b:
    print(8)
