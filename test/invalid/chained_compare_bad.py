# Chained comparison with incompatible types
def main() -> None:
    x: int = 5
    y: str = "hello"
    if 1 < x < y:
        print(1)
    print(0)
