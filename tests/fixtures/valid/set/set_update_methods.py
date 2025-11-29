# Test set update methods

# pop()
s1: set[int] = {1, 2, 3}
print(len(s1))
v: int = s1.pop()
print(len(s1))

# update() - in-place union
s2: set[int] = {1, 2}
s3: set[int] = {3, 4}
s2.update(s3)
print(len(s2))
print(1 in s2)
print(3 in s2)
print(4 in s2)

# difference_update() - in-place difference
s4: set[int] = {1, 2, 3, 4, 5}
s5: set[int] = {2, 4}
s4.difference_update(s5)
print(len(s4))
print(1 in s4)
print(2 in s4)
print(3 in s4)

# intersection_update() - in-place intersection
s6: set[int] = {1, 2, 3, 4, 5}
s7: set[int] = {2, 4, 6}
s6.intersection_update(s7)
print(len(s6))
print(2 in s6)
print(4 in s6)
print(1 in s6)

# symmetric_difference_update() - in-place symmetric diff
s8: set[int] = {1, 2, 3}
s9: set[int] = {2, 3, 4}
s8.symmetric_difference_update(s9)
print(len(s8))
print(1 in s8)
print(4 in s8)
print(2 in s8)
