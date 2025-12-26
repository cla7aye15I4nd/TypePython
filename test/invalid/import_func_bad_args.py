# Call function with wrong argument count (local function)
def square(n: int) -> int:
    return n * n

def main() -> None:
    x: int = square(1, 2, 3)
    print(x)
