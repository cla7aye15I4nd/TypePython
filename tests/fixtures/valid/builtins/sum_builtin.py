# sum builtin
lst: list[int] = [1, 2, 3, 4, 5]
print(sum(lst))

# Empty list
empty: list[int] = []
print(sum(empty))

# With start value
print(sum(lst, 10))
