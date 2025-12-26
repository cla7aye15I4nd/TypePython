# len() with wrong argument count
def main() -> None:
    nums: list[int] = [1, 2, 3]
    x: int = len(nums, 5)
    print(x)
