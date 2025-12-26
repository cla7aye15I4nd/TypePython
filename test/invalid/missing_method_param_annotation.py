# Method parameter without type annotation
class Calculator:
    value: int

    def __init__(self) -> None:
        self.value = 0

    def add(self, x) -> int:
        return self.value + x

def main() -> None:
    c: Calculator = Calculator()
    print(c.add(5))
