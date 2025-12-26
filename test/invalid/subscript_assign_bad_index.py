# Subscript assignment with non-integer index
def main() -> None:
    nums: list[int] = [1, 2, 3]
    nums["hello"] = 5
    print(nums[0])
