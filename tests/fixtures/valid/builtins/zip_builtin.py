# zip builtin
lst1: list[int] = [1, 2, 3]
lst2: list[str] = ["a", "b", "c"]
for x, y in zip(lst1, lst2):
    print(x, y)
