# Test set discard() method
# discard() removes element if present, does nothing if not (no error)

# Basic discard
s1: set[int] = {1, 2, 3, 4, 5}
print(len(s1))
s1.discard(3)
print(len(s1))
b1: bool = 3 in s1
print(b1)

# Discard non-existent element (should not raise error)
s1.discard(99)
print(len(s1))

# Discard first element
s2: set[int] = {10, 20, 30}
s2.discard(10)
print(len(s2))
b2: bool = 10 in s2
print(b2)

# Discard last element
s3: set[int] = {100, 200, 300}
s3.discard(300)
print(len(s3))
b3: bool = 300 in s3
print(b3)

# Discard all elements one by one
s4: set[int] = {1, 2, 3}
s4.discard(1)
s4.discard(2)
s4.discard(3)
print(len(s4))

# Discard from empty set (should not error)
s5: set[int] = set()
s5.discard(42)
print(len(s5))

# Discard same element multiple times
s6: set[int] = {7, 8, 9}
s6.discard(8)
s6.discard(8)
s6.discard(8)
print(len(s6))
b4: bool = 8 in s6
print(b4)

# Verify remaining elements
b5: bool = 7 in s6
print(b5)
b6: bool = 9 in s6
print(b6)
