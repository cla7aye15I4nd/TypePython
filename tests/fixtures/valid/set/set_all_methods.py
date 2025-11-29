# Comprehensive test of all set methods in one file

# Test set copy
s1: set[int] = {1, 2, 3}
s2: set[int] = s1.copy()
print(len(s1))
print(len(s2))
s1.add(4)
print(len(s1))
print(len(s2))  # Should remain 3
b1: bool = 4 in s1
print(b1)
b2: bool = 4 in s2
print(b2)

# Test set union
s3: set[int] = {1, 2, 3}
s4: set[int] = {3, 4, 5}
s5: set[int] = s3.union(s4)
print(len(s5))  # 5: {1, 2, 3, 4, 5}
b3: bool = 1 in s5
print(b3)
b4: bool = 5 in s5
print(b4)

# Test set intersection
s6: set[int] = {1, 2, 3, 4}
s7: set[int] = {3, 4, 5, 6}
s8: set[int] = s6.intersection(s7)
print(len(s8))  # 2: {3, 4}
b5: bool = 3 in s8
print(b5)
b6: bool = 4 in s8
print(b6)
b7: bool = 1 in s8
print(b7)
b8: bool = 5 in s8
print(b8)

# Test set difference
s9: set[int] = {1, 2, 3, 4}
s10: set[int] = {3, 4, 5, 6}
s11: set[int] = s9.difference(s10)
print(len(s11))  # 2: {1, 2}
b9: bool = 1 in s11
print(b9)
b10: bool = 2 in s11
print(b10)
b11: bool = 3 in s11
print(b11)
b12: bool = 4 in s11
print(b12)

# Test set symmetric_difference
s12: set[int] = set({1, 2, 3, 4})
s13: set[int] = {3, 4, 5, 6}
s14: set[int] = s12.symmetric_difference(s13)
print(len(s14))  # 4: {1, 2, 5, 6}
b13: bool = 1 in s14
print(b13)
b14: bool = 2 in s14
print(b14)
b15: bool = 3 in s14
print(b15)
b16: bool = 4 in s14
print(b16)
b17: bool = 5 in s14
print(b17)
b18: bool = 6 in s14
print(b18)

# Test copy with empty set
empty1: set[int] = set()
empty2: set[int] = empty1.copy()
print(len(empty2))

# Test union with empty
s15: set[int] = {1, 2}
empty3: set[int] = set()
s16: set[int] = s15.union(empty3)
print(len(s16))

# Test intersection with empty
s17: set[int] = {1, 2}
empty4: set[int] = set()
s18: set[int] = s17.intersection(empty4)
print(len(s18))

# Test difference with empty
s19: set[int] = {1, 2}
empty5: set[int] = set()
s20: set[int] = s19.difference(empty5)
print(len(s20))

# Test symmetric_difference with empty
s21: set[int] = {1, 2}
empty6: set[int] = set()
s22: set[int] = s21.symmetric_difference(empty6)
print(len(s22))

# Test union with identical sets
s23: set[int] = {1, 2, 3}
s24: set[int] = {1, 2, 3}
s25: set[int] = s23.union(s24)
print(len(s25))

# Test intersection with identical sets
s26: set[int] = {1, 2, 3}
s27: set[int] = {1, 2, 3}
s28: set[int] = s26.intersection(s27)
print(len(s28))

# Test difference with identical sets
s29: set[int] = {1, 2, 3}
s30: set[int] = {1, 2, 3}
s31: set[int] = s29.difference(s30)
print(len(s31))

# Test symmetric_difference with identical sets
s32: set[int] = {1, 2, 3}
s33: set[int] = {1, 2, 3}
s34: set[int] = s32.symmetric_difference(s33)
print(len(s34))

# Test union with disjoint sets
s35: set[int] = {1, 2, 3}
s36: set[int] = {4, 5, 6}
s37: set[int] = s35.union(s36)
print(len(s37))

# Test intersection with disjoint sets
s38: set[int] = {1, 2, 3}
s39: set[int] = {4, 5, 6}
s40: set[int] = s38.intersection(s39)
print(len(s40))

# Test chaining operations
s41: set[int] = {1, 2, 3}
s42: set[int] = {2, 3, 4}
s43: set[int] = {3, 4, 5}
s44: set[int] = s41.union(s42).union(s43)
print(len(s44))

# Test copy doesn't affect original
s45: set[int] = {10, 20, 30}
s46: set[int] = s45.copy()
s46.add(40)
print(len(s45))
print(len(s46))
