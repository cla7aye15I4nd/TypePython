# Test class that is iterable (returns separate iterator)

class NumberIterator:
    def __init__(self, numbers: list[int]) -> None:
        self.numbers: list[int] = numbers
        self.index: int = 0

    def __next__(self) -> int:
        if self.index >= len(self.numbers):
            raise StopIteration
        value: int = self.numbers[self.index]
        self.index = self.index + 1
        return value

class NumberCollection:
    def __init__(self) -> None:
        self.data: list[int] = []

    def add(self, n: int) -> None:
        self.data.append(n)

    def __iter__(self) -> NumberIterator:
        return NumberIterator(self.data)

# Use iterable class
coll: NumberCollection = NumberCollection()
coll.add(10)
coll.add(20)
coll.add(30)

for num in coll:
    print(b"Number:", num)

# Can iterate multiple times (fresh iterator each time)
print(b"Second iteration:")
for num in coll:
    print(b"Number:", num)
