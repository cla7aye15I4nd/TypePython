# Generator with list operations - tests various expression types
def gen_list_items() -> int:
    lst: list[int] = [10, 20, 30]
    i: int = 0
    while i < len(lst):
        yield lst[i]
        i = i + 1

for item in gen_list_items():
    print(item)
