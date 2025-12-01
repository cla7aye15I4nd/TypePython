# Iteration over dict.values() with string keys
d: dict[str, int] = {"a": 1, "b": 2, "c": 3}
total: int = 0
for v in d.values():
    total = total + v
print(total)
