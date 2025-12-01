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

# Float lists
float_lst1: list[float] = [1.0, 2.0, 3.0]
float_lst2: list[float] = [0.0, 0.0, 0.0]
float_lst3: list[float] = [1.0, 0.0, 2.0]

print(all(float_lst1))
print(all(float_lst2))
print(all(float_lst3))

print(any(float_lst1))
print(any(float_lst2))
print(any(float_lst3))

# String lists
str_lst1: list[str] = ["a", "b", "c"]
str_lst2: list[str] = ["", "", ""]
str_lst3: list[str] = ["a", "", "c"]

print(all(str_lst1))
print(all(str_lst2))
print(all(str_lst3))

print(any(str_lst1))
print(any(str_lst2))
print(any(str_lst3))

# Dict with int keys
d1: dict[int, int] = {1: 10, 2: 20}
d2: dict[int, int] = {0: 10, 0: 20}
d_empty: dict[int, int] = {}

print(all(d1))
print(any(d1))
print(all(d_empty))
print(any(d_empty))
