# Test for loop over lists

# Iterate over list of integers
nums: list[int] = [10, 20, 30, 40, 50]
total: int = 0
for n in nums:
    total = total + n
print(b"Sum:", total)

# Iterate over list of strings
names: list[str] = ["alice", "bob", "charlie"]
for name in names:
    print(b"Hello", name)

# Iterate over list of floats
values: list[float] = [1.5, 2.5, 3.5]
sum_f: float = 0.0
for v in values:
    sum_f = sum_f + v
print(b"Float sum:", sum_f)

# Iterate over list of bools
flags: list[bool] = [True, False, True, True]
true_count: int = 0
for f in flags:
    if f:
        true_count = true_count + 1
print(b"True count:", true_count)

# Modify list during iteration (collect results)
squares: list[int] = []
for i in range(5):
    squares.append(i * i)
print(b"Squares:", squares)
