# Iteration over dict.items() with string keys and unpacking
d: dict[str, int] = {"a": 1, "b": 2, "c": 3}
for k, v in d.items():
    print(k)
    print(v)
