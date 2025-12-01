# Set methods
s: set[int] = {1, 2, 3}

# add
s.add(4)
print(len(s))

# remove
s.remove(2)
print(len(s))

# discard
s.discard(1)
print(len(s))

# pop
val: int = s.pop()
print(len(s))

# clear
s2: set[int] = {10, 20, 30}
s2.clear()
print(len(s2))

# copy
s3: set[int] = {100, 200}
s4: set[int] = s3.copy()
print(len(s4))

# union
s5: set[int] = {1, 2}
s6: set[int] = {2, 3}
s7: set[int] = s5.union(s6)
print(len(s7))

# intersection
s8: set[int] = s5.intersection(s6)
print(len(s8))

# difference
s9: set[int] = s5.difference(s6)
print(len(s9))
