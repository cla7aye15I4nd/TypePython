# Test class methods that return iterables

class DataStore:
    items: list[str]
    counts: dict[str, int]

    def __init__(self) -> None:
        self.items: list[str] = []
        self.counts: dict[str, int] = {}

    def add_item(self, name: str, count: int) -> None:
        self.items.append(name)
        self.counts[name] = count

    def get_items(self) -> list[str]:
        return self.items

    def get_counts(self) -> dict[str, int]:
        return self.counts

    def item_count_pairs(self) -> list[tuple[str, int]]:
        result: list[tuple[str, int]] = []
        for item in self.items:
            result.append((item, self.counts[item]))
        return result

# Use class methods that return iterables
store: DataStore = DataStore()
store.add_item("apple", 5)
store.add_item("banana", 3)
store.add_item("cherry", 7)

# Iterate over returned list
print(b"Items:")
for item in store.get_items():
    print(b"  -", item)

# Iterate over returned dict
print(b"Counts:")
for name, count in store.get_counts().items():
    print(b"  ", name, b":", count)

# Iterate over list of tuples
print(b"Pairs:")
for name, count in store.item_count_pairs():
    print(b"  ", name, b"has", count)
