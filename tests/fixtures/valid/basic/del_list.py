# del statement on list item
lst: list[int] = [10, 20, 30, 40, 50]
print(len(lst))
del lst[2]
print(len(lst))
print(lst[2])
