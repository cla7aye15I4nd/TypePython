# Test for loop over sets

# Iterate over set of integers
nums: set[int] = {1, 2, 3, 4, 5}
total: int = 0
for n in nums:
    total = total + n
print(b"Sum:", total)

# Iterate over set of strings - collect and sort for deterministic order
colors: set[str] = {"red", "green", "blue"}
color_list: list[str] = []
for color in colors:
    color_list.append(color)
color_list.sort()
for c in color_list:
    print(b"Color:", c)

# Empty set iteration
empty: set[int] = set()
count: int = 0
for x in empty:
    count = count + 1
print(b"Empty set iterations:", count)
