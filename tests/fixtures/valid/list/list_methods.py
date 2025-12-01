# List methods
lst: list[int] = [1, 2, 3]

# append
lst.append(4)
print(lst)

# pop
x: int = lst.pop()
print(x)
print(lst)

# insert
lst.insert(0, 10)
print(lst)

# remove
lst.remove(2)
print(lst)

# clear
lst2: list[int] = [1, 2, 3]
lst2.clear()
print(len(lst2))

# copy
lst3: list[int] = [1, 2, 3]
lst4: list[int] = lst3.copy()
print(lst4)

# reverse
lst5: list[int] = [3, 1, 2]
lst5.reverse()
print(lst5)

# sort
lst6: list[int] = [3, 1, 2]
lst6.sort()
print(lst6)

# index
lst7: list[int] = [10, 20, 30]
print(lst7.index(20))

# count
lst8: list[int] = [1, 2, 1, 3, 1]
print(lst8.count(1))

# extend
lst9: list[int] = [1, 2]
lst9.extend([3, 4, 5])
print(lst9)
