# Test string % formatting with dict
# Only check length since dict ordering is non-deterministic

fmt: str = "Dict: %s"
d: dict[int, int] = {1: 10}
result: str = fmt % d
print(len(result))  # 13 = len("Dict: {1: 10}")

# Empty dict
d2: dict[int, int] = dict()
result2: str = fmt % d2
print(result2)  # "Dict: {}" is deterministic

print("dict format test passed!")
