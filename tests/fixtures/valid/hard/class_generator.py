# Test class-based generators and advanced iterator patterns

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

# Fibonacci sequence
fib: Fibonacci = Fibonacci(10)
for n in fib:
    print(b"Fib:", n)

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

# Countdown
for n in Countdown(5):
    print(b"Countdown:", n)

class Repeater:
    value: str
    times: int
    count: int

    def __init__(self, value: str, times: int) -> None:
        self.value: str = value
        self.times: int = times
        self.count: int = 0

    def __iter__(self) -> 'Repeater':
        return self

    def __next__(self) -> str:
        if self.count >= self.times:
            raise StopIteration
        self.count = self.count + 1
        return self.value

# Repeat value
for s in Repeater("hello", 3):
    print(b"Repeated:", s)

class Cycle:
    items: list[int]
    times: int
    current_cycle: int
    index: int

    def __init__(self, items: list[int], times: int) -> None:
        self.items: list[int] = items
        self.times: int = times
        self.current_cycle: int = 0
        self.index: int = 0

    def __iter__(self) -> 'Cycle':
        return self

    def __next__(self) -> int:
        if self.current_cycle >= self.times:
            raise StopIteration
        val: int = self.items[self.index]
        self.index = self.index + 1
        if self.index >= len(self.items):
            self.index = 0
            self.current_cycle = self.current_cycle + 1
        return val

# Cycle through list
for n in Cycle([1, 2, 3], 2):
    print(b"Cycled:", n)
