# 'not' operator on non-boolean type (not bool or int)
def main() -> None:
    x: str = "hello"
    y: bool = not x
    print(y)
