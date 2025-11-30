# Test container in container operations
# These check if a container is an element of another container
# For int collections, this is always False since we don't support nested collections

# list in list
a: list[int] = [1, 2, 3]
b: list[int] = [1, 2, 3, 4, 5]
result1: bool = a in b
print(result1)  # False

result2: bool = a not in b
print(result2)  # True

# set in list
s: set[int] = {1, 2, 3}
l: list[int] = [1, 2, 3, 4, 5]
result3: bool = s in l
print(result3)  # False

result4: bool = s not in l
print(result4)  # True

# set in set
s1: set[int] = {1, 2}
s2: set[int] = {1, 2, 3, 4}
result5: bool = s1 in s2
print(result5)  # False

result6: bool = s1 not in s2
print(result6)  # True

# dict in list
d: dict[int, int] = {1: 10}
dl: list[int] = [1, 2, 3]
result7: bool = d in dl
print(result7)  # False

result8: bool = d not in dl
print(result8)  # True

print("container in container tests passed!")
