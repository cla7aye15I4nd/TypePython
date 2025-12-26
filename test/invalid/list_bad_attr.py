# Invalid list attribute access
def main() -> None:
    nums: list[int] = [1, 2, 3]
    x: int = nums.nonexistent
    print(x)
