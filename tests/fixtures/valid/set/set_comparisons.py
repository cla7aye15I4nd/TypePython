# Test set comparison methods: issubset, issuperset, isdisjoint

# Test issubset
s1: set[int] = {1, 2, 3}
s2: set[int] = {1, 2, 3, 4, 5}

b1: bool = s1.issubset(s2)
print(b1)  # True - s1 is subset of s2

b2: bool = s2.issubset(s1)
print(b2)  # False - s2 is not subset of s1

# Subset of itself
b3: bool = s1.issubset(s1)
print(b3)  # True

# Empty set is subset of any set
empty: set[int] = set()
b4: bool = empty.issubset(s1)
print(b4)  # True

# Test issuperset
b5: bool = s2.issuperset(s1)
print(b5)  # True - s2 is superset of s1

b6: bool = s1.issuperset(s2)
print(b6)  # False - s1 is not superset of s2

# Superset of itself
b7: bool = s1.issuperset(s1)
print(b7)  # True

# Any set is superset of empty set
b8: bool = s1.issuperset(empty)
print(b8)  # True

# Test isdisjoint
s3: set[int] = {1, 2, 3}
s4: set[int] = {4, 5, 6}
s5: set[int] = {3, 4, 5}

b9: bool = s3.isdisjoint(s4)
print(b9)  # True - no common elements

b10: bool = s3.isdisjoint(s5)
print(b10)  # False - have element 3 in common

b11: bool = s3.isdisjoint(s3)
print(b11)  # False - same set, not disjoint

# Empty set is disjoint with any set
b12: bool = empty.isdisjoint(s1)
print(b12)  # True

b13: bool = s1.isdisjoint(empty)
print(b13)  # True

# More complex issubset cases
s6: set[int] = {1, 2}
s7: set[int] = {1, 2, 3, 4}
s8: set[int] = {1, 3}

b14: bool = s6.issubset(s7)
print(b14)  # True

b15: bool = s8.issubset(s7)
print(b15)  # True

b16: bool = s6.issubset(s8)
print(b16)  # False - s6 has 2, s8 doesn't

# Chain comparisons
s9: set[int] = {1}
s10: set[int] = {1, 2}
s11: set[int] = {1, 2, 3}

b17: bool = s9.issubset(s10)
print(b17)  # True

b18: bool = s10.issubset(s11)
print(b18)  # True

b19: bool = s11.issuperset(s10)
print(b19)  # True

b20: bool = s10.issuperset(s9)
print(b20)  # True
