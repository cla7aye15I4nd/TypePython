# Subscript access with non-integer index
def main() -> None:
    nums: list[int] = [1, 2, 3]
    x: int = nums["hello"]
    print(x)
