# Test set methods

s1: set[int] = {1, 2, 3, 4, 5}
s2: set[int] = {3, 4, 5, 6, 7}

# Test copy
s3: set[int] = s1.copy()
print(len(s3))
print(1 in s3)
print(5 in s3)

# Test union method
s4: set[int] = s1.union(s2)
print(len(s4))

# Test intersection method
s5: set[int] = s1.intersection(s2)
print(len(s5))

# Test difference method
s6: set[int] = s1.difference(s2)
print(len(s6))

# Test symmetric_difference method
s7: set[int] = s1.symmetric_difference(s2)
print(len(s7))

# Test issubset
s8: set[int] = {1, 2}
b1: bool = s8.issubset(s1)
print(b1)

b2: bool = s1.issubset(s8)
print(b2)

# Test issuperset
b3: bool = s1.issuperset(s8)
print(b3)

b4: bool = s8.issuperset(s1)
print(b4)

# Test isdisjoint
s9: set[int] = {10, 20, 30}
b5: bool = s1.isdisjoint(s9)
print(b5)

b6: bool = s1.isdisjoint(s2)
print(b6)
