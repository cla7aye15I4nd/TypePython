# any and all builtins
lst1: list[bool] = [True, True, True]
lst2: list[bool] = [True, False, True]
lst3: list[bool] = [False, False, False]

print(all(lst1))
print(all(lst2))
print(all(lst3))

print(any(lst1))
print(any(lst2))
print(any(lst3))
