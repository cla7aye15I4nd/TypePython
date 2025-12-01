# Test bytes binary operators - comparisons, membership, identity, logical

# Test bytes comparison operators
a: bytes = b"hello"
b: bytes = b"hello"
c: bytes = b"world"
d: bytes = b"abc"
e: bytes = b"xyz"

# Equality
print(a == b)  # True
print(a == c)  # False
print(a != c)  # True
print(a != b)  # False

# Less than / greater than
print(d < e)   # True (abc < xyz)
print(e < d)   # False
print(d <= e)  # True
print(d <= d)  # True (equal)
print(e > d)   # True
print(d > e)   # False
print(e >= d)  # True
print(e >= e)  # True (equal)

# Membership - bytes in bytes
haystack: bytes = b"hello world"
needle1: bytes = b"world"
needle2: bytes = b"xyz"

print(needle1 in haystack)      # True
print(needle2 in haystack)      # False
print(needle1 not in haystack)  # False
print(needle2 not in haystack)  # True

# Identity operators
same1: bytes = b"test"
same2: bytes = b"test"
print(same1 is same2)      # may be True (string interning) or False
print(same1 is not same2)  # opposite

# Concatenation
f: bytes = b"foo"
g: bytes = b"bar"
result: bytes = f + g
print(len(result))  # 6

# Repetition
h: bytes = b"ab"
repeated: bytes = h * 3
print(len(repeated))  # 6

# Bool repetition
rep_bool: bytes = h * True
print(len(rep_bool))  # 2

# Logical operators bytes and/or bytes
empty: bytes = b""
nonempty: bytes = b"hi"

# and: returns first falsy or last truthy
and_result1: bytes = nonempty and b"there"  # b"there"
print(len(and_result1))  # 5

and_result2: bytes = empty and b"there"  # b""
print(len(and_result2))  # 0

# or: returns first truthy or last
or_result1: bytes = nonempty or b"there"  # b"hi"
print(len(or_result1))  # 2

or_result2: bytes = empty or b"there"  # b"there"
print(len(or_result2))  # 5

# Mixed type and/or (returns bool)
and_mixed: bool = nonempty and True
print(and_mixed)  # True

or_mixed: bool = empty or False
print(or_mixed)  # False

# Unary not on bytes
print(not empty)     # True (empty is falsy)
print(not nonempty)  # False (non-empty is truthy)
