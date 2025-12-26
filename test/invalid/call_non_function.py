# Calling non-function type
def main() -> None:
    x: int = 5
    y: int = x()
    print(y)
