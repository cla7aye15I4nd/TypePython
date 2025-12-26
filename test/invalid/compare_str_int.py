# Compare incompatible types - string and int
def main() -> None:
    x: str = "hello"
    y: int = 5
    if x < y:
        print(1)
    print(0)
