# Test for loop over sets

# Iterate over set of integers
nums: set[int] = {1, 2, 3, 4, 5}
total: int = 0
for n in nums:
    total = total + n
print(b"Sum:", total)

# Iterate over set of strings
colors: set[str] = {"red", "green", "blue"}
for color in colors:
    print(b"Color:", color)

# Empty set iteration
empty: set[int] = set()
count: int = 0
for x in empty:
    count = count + 1
print(b"Empty set iterations:", count)
