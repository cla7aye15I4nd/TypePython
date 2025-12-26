# Attribute assignment on non-instance type
def main() -> None:
    x: int = 5
    x.value = 10
    print(x)
