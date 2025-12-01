# Test custom class iterator protocol

class Counter:
    current: int
    stop: int

    def __init__(self, start: int, stop: int) -> None:
        self.current: int = start
        self.stop: int = stop

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

# Use iter/next on custom class
counter2: Counter = Counter(10, 13)
it = iter(counter2)
print(b"Manual:", next(it))
print(b"Manual:", next(it))
print(b"Manual:", next(it))
