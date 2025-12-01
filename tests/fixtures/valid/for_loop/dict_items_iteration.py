# Iteration over dict.items() with tuple unpacking
d: dict[int, int] = {1: 10, 2: 20, 3: 30}
key_sum: int = 0
val_sum: int = 0
for k, v in d.items():
    key_sum = key_sum + k
    val_sum = val_sum + v
print(key_sum)
print(val_sum)
# Expected: 6, 60
