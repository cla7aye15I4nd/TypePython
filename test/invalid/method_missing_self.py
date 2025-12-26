# Method without 'self' as first parameter
class Counter:
    count: int

    def increment(x: int) -> int:
        return x + 1

def main() -> None:
    c: Counter = Counter()
    print(c.increment(1))
