# Function with slice expression using stop - exercises stop in Slice expr
def slice_fn() -> int:
    lst: list[int] = [1, 2, 3, 4, 5]
    sliced: list[int] = lst[1:3]  # Has both start and stop
    total: int = 0
    for x in sliced:
        total = total + x
    return total

print(slice_fn())
