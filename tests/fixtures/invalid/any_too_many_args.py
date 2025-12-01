# any() takes exactly 1 argument
lst1: list[int] = [1, 2, 3]
lst2: list[int] = [4, 5, 6]
result: bool = any(lst1, lst2)
print(result)
