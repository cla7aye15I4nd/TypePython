# Test functions with container parameters

def process_list(items: list[int]) -> int:
    return len(items)

def process_dict(mapping: dict[int, int]) -> int:
    return len(mapping)

def process_set(items: set[int]) -> int:
    return len(items)

# Test calling these functions
l: list[int] = [1, 2, 3, 4]
print(process_list(l))  # 4

d: dict[int, int] = {1: 10, 2: 20, 3: 30}
print(process_dict(d))  # 3

s: set[int] = {1, 2, 3, 4, 5}
print(process_set(s))  # 5

print("container params test passed!")
