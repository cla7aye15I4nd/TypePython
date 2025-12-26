# Boolean operation on non-boolean types (not bool or int)
def main() -> None:
    x: str = "hello"
    y: bool = x and True
    print(y)
