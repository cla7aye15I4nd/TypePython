# Test set methods: copy, union, intersection, difference, symmetric_difference
# Also tests issubset, issuperset, isdisjoint

# Test sets
a: set[int] = {1, 2, 3}
b: set[int] = {2, 3, 4}
c: set[int] = {5, 6}

# copy - returns a copy of the set
copy_a: set[int] = a.copy()
print(1)  # copy_a should have 1
print(2)  # copy_a should have 2
print(3)  # copy_a should have 3

# union - returns set with all elements from both sets
union_ab: set[int] = a.union(b)
print(len(union_ab))  # {1, 2, 3, 4} = 4 elements

# intersection - returns set with common elements
inter_ab: set[int] = a.intersection(b)
print(len(inter_ab))  # {2, 3} = 2 elements

# difference - returns set with elements in a but not in b
diff_ab: set[int] = a.difference(b)
print(len(diff_ab))  # {1} = 1 element

# symmetric_difference - returns set with elements in either but not both
sym_diff_ab: set[int] = a.symmetric_difference(b)
print(len(sym_diff_ab))  # {1, 4} = 2 elements

# issubset - returns True if all elements of a are in b
sub1: bool = a.issubset(b)
print(sub1)  # False

d: set[int] = {1, 2}
sub2: bool = d.issubset(a)
print(sub2)  # True

# issuperset - returns True if all elements of b are in a
sup1: bool = a.issuperset(b)
print(sup1)  # False

sup2: bool = a.issuperset(d)
print(sup2)  # True

# isdisjoint - returns True if sets have no common elements
dis1: bool = a.isdisjoint(c)
print(dis1)  # True

dis2: bool = a.isdisjoint(b)
print(dis2)  # False
