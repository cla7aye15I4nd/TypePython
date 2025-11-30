# Test functions that return container types

def make_list() -> list[int]:
    result: list[int] = [1, 2, 3]
    return result

def make_dict() -> dict[int, int]:
    result: dict[int, int] = {1: 10, 2: 20}
    return result

def make_set() -> set[int]:
    result: set[int] = {1, 2, 3}
    return result

# Test calling these functions
l: list[int] = make_list()
print(len(l))  # 3

d: dict[int, int] = make_dict()
print(len(d))  # 2

s: set[int] = make_set()
print(len(s))  # 3

print("return containers test passed!")
