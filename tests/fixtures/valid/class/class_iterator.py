# Test custom class iterator protocol
class Counter:
    current: int
    stop: int

    def __init__(self, start: int, stop: int) -> None:
        self.current = start
        self.stop = stop

    def __iter__(self) -> 'Counter':
        return self

    def __next__(self) -> int:
        if self.current >= self.stop:
            raise StopIteration
        value: int = self.current
        self.current = self.current + 1
        return value

# Use custom iterator in for loop
counter: Counter = Counter(0, 5)
for n in counter:
    print(b"Count:", n)
