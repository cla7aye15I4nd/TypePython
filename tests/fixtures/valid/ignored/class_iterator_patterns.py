# Test comprehensive class iterator patterns

# Simple counter iterator
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
        val: int = self.current
        self.current = self.current + 1
        return val

for n in Counter(0, 5):
    print(b"Counter:", n)

# Step iterator
class StepCounter:
    current: int
    stop: int
    step: int

    def __init__(self, start: int, stop: int, step: int) -> None:
        self.current: int = start
        self.stop: int = stop
        self.step: int = step

    def __iter__(self) -> 'StepCounter':
        return self

    def __next__(self) -> int:
        if self.step > 0 and self.current >= self.stop:
            raise StopIteration
        if self.step < 0 and self.current <= self.stop:
            raise StopIteration
        val: int = self.current
        self.current = self.current + self.step
        return val

for n in StepCounter(0, 10, 2):
    print(b"Step:", n)

for n in StepCounter(10, 0, -2):
    print(b"Neg step:", n)

# Fibonacci iterator
class Fibonacci:
    limit: int
    a: int
    b: int
    count: int

    def __init__(self, limit: int) -> None:
        self.limit: int = limit
        self.a: int = 0
        self.b: int = 1
        self.count: int = 0

    def __iter__(self) -> 'Fibonacci':
        return self

    def __next__(self) -> int:
        if self.count >= self.limit:
            raise StopIteration
        result: int = self.a
        self.a, self.b = self.b, self.a + self.b
        self.count = self.count + 1
        return result

for n in Fibonacci(10):
    print(b"Fib:", n)

# Repeat iterator
class Repeat:
    value: str
    times: int
    count: int

    def __init__(self, value: str, times: int) -> None:
        self.value: str = value
        self.times: int = times
        self.count: int = 0

    def __iter__(self) -> 'Repeat':
        return self

    def __next__(self) -> str:
        if self.count >= self.times:
            raise StopIteration
        self.count = self.count + 1
        return self.value

for s in Repeat("hello", 3):
    print(b"Repeat:", s)

# Cycle iterator
class Cycle:
    items: list[int]
    times: int
    cycle: int
    index: int

    def __init__(self, items: list[int], times: int) -> None:
        self.items: list[int] = items
        self.times: int = times
        self.cycle: int = 0
        self.index: int = 0

    def __iter__(self) -> 'Cycle':
        return self

    def __next__(self) -> int:
        if self.cycle >= self.times:
            raise StopIteration
        val: int = self.items[self.index]
        self.index = self.index + 1
        if self.index >= len(self.items):
            self.index = 0
            self.cycle = self.cycle + 1
        return val

for n in Cycle([1, 2, 3], 2):
    print(b"Cycle:", n)

# Filter iterator
class FilterIter:
    data: list[int]
    threshold: int
    index: int

    def __init__(self, data: list[int], threshold: int) -> None:
        self.data: list[int] = data
        self.threshold: int = threshold
        self.index: int = 0

    def __iter__(self) -> 'FilterIter':
        return self

    def __next__(self) -> int:
        while self.index < len(self.data):
            val: int = self.data[self.index]
            self.index = self.index + 1
            if val > self.threshold:
                return val
        raise StopIteration

for n in FilterIter([1, 5, 2, 8, 3, 9, 4], 4):
    print(b"Filtered:", n)

# Map iterator
class MapIter:
    data: list[int]
    index: int

    def __init__(self, data: list[int]) -> None:
        self.data: list[int] = data
        self.index: int = 0

    def __iter__(self) -> 'MapIter':
        return self

    def __next__(self) -> int:
        if self.index >= len(self.data):
            raise StopIteration
        val: int = self.data[self.index]
        self.index = self.index + 1
        return val * val

for n in MapIter([1, 2, 3, 4, 5]):
    print(b"Mapped:", n)

# Separate iterator from iterable
class NumberIterator:
    numbers: list[int]
    index: int

    def __init__(self, numbers: list[int]) -> None:
        self.numbers: list[int] = numbers
        self.index: int = 0

    def __next__(self) -> int:
        if self.index >= len(self.numbers):
            raise StopIteration
        val: int = self.numbers[self.index]
        self.index = self.index + 1
        return val

class NumberCollection:
    data: list[int]

    def __init__(self) -> None:
        self.data: list[int] = []

    def add(self, n: int) -> None:
        self.data.append(n)

    def __iter__(self) -> NumberIterator:
        return NumberIterator(self.data)

coll: NumberCollection = NumberCollection()
coll.add(10)
coll.add(20)
coll.add(30)

for n in coll:
    print(b"Coll:", n)

# Can iterate multiple times
for n in coll:
    print(b"Coll again:", n)

# Grid iterator
class GridIterator:
    rows: int
    cols: int
    row: int
    col: int

    def __init__(self, rows: int, cols: int) -> None:
        self.rows: int = rows
        self.cols: int = cols
        self.row: int = 0
        self.col: int = 0

    def __iter__(self) -> 'GridIterator':
        return self

    def __next__(self) -> tuple[int, int]:
        if self.row >= self.rows:
            raise StopIteration
        result: tuple[int, int] = (self.row, self.col)
        self.col = self.col + 1
        if self.col >= self.cols:
            self.col = 0
            self.row = self.row + 1
        return result

for pos in GridIterator(3, 4):
    print(b"Grid:", pos)

# Countdown iterator
class Countdown:
    current: int

    def __init__(self, start: int) -> None:
        self.current: int = start

    def __iter__(self) -> 'Countdown':
        return self

    def __next__(self) -> int:
        if self.current < 0:
            raise StopIteration
        val: int = self.current
        self.current = self.current - 1
        return val

for n in Countdown(5):
    print(b"Countdown:", n)

# Prime iterator
class Primes:
    limit: int
    current: int

    def __init__(self, limit: int) -> None:
        self.limit: int = limit
        self.current: int = 2

    def __iter__(self) -> 'Primes':
        return self

    def _is_prime(self, n: int) -> bool:
        if n < 2:
            return False
        for i in range(2, n):
            if n % i == 0:
                return False
        return True

    def __next__(self) -> int:
        while self.current <= self.limit:
            if self._is_prime(self.current):
                val: int = self.current
                self.current = self.current + 1
                return val
            self.current = self.current + 1
        raise StopIteration

for p in Primes(30):
    print(b"Prime:", p)
