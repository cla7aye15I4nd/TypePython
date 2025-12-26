# Missing return value from non-void function
def foo() -> int:
    x: int = 5
    return

def main() -> None:
    print(foo())
