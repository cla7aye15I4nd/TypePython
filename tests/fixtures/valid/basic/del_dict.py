# del statement on dict item
d: dict[int, int] = {1: 10, 2: 20, 3: 30}
print(len(d))
del d[2]
print(len(d))
print(2 in d)
