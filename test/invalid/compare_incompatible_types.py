# Comparing incompatible types
def main() -> None:
    x: str = "hello"
    y: int = 5
    if x == y:
        print(1)
    else:
        print(0)
