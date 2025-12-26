# HashSet: Simple set using a list

class HashSet:
    items: list[int]
    size: int

    def __init__(self) -> None:
        self.items = [0]
        self.size = 0

    def add(self, value: int) -> None:
        # Only add if not already present
        if self.contains(value) == 0:
            if self.size == 0:
                self.items[0] = value
            else:
                self.items.append(value)
            self.size = self.size + 1

    def contains(self, value: int) -> int:
        i: int = 0
        while i < self.size:
            if self.items[i] == value:
                return 1
            i = i + 1
        return 0

    def get_size(self) -> int:
        return self.size


def test_hashset_basic() -> int:
    s: HashSet = HashSet()
    s.add(1)
    s.add(2)
    s.add(3)
    s.add(2)  # Duplicate, should not increase size
    return s.get_size()  # Expected: 3


def test_hashset_contains() -> int:
    s: HashSet = HashSet()
    s.add(10)
    s.add(20)
    s.add(30)
    result: int = s.contains(10) + s.contains(20) + s.contains(40)
    return result  # Expected: 2
