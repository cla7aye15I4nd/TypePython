# Test del statement for dict and list

# Test del on dict
d: dict[int, int] = {1: 10, 2: 20, 3: 30}
print(len(d))
del d[2]
print(len(d))
print(d[1])
print(d[3])

# Test del on list
lst: list[int] = [1, 2, 3, 4, 5]
print(len(lst))
del lst[2]
print(len(lst))
print(lst[0])
print(lst[1])
print(lst[2])
print(lst[3])

# Test del on list with negative index
lst2: list[int] = [10, 20, 30, 40]
del lst2[-1]
print(len(lst2))
print(lst2[0])
print(lst2[1])
print(lst2[2])

# Test del on dict multiple times
d2: dict[int, int] = {1: 100, 2: 200, 3: 300, 4: 400}
del d2[1]
del d2[3]
print(len(d2))
print(d2[2])
print(d2[4])
