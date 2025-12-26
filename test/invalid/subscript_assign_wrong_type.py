# Subscript assignment with wrong element type
def main() -> None:
    nums: list[int] = [1, 2, 3]
    nums[0] = "hello"
    print(nums[0])
