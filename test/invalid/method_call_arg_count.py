# Method call with wrong argument count
class Calculator:
    value: int

    def __init__(self) -> None:
        self.value = 0

    def add(self, x: int) -> int:
        return self.value + x

def main() -> None:
    c: Calculator = Calculator()
    result: int = c.add(1, 2, 3)
    print(result)
