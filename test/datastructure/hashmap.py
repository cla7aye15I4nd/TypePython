# HashMap: Simple hash map using parallel lists for keys and values

class HashMap:
    keys: list[int]
    values: list[int]
    size: int

    def __init__(self) -> None:
        print("HashMap created")
        self.keys = [0]
        self.values = [0]
        self.size = 0

    def __str__(self) -> str:
        return "HashMap(size)"

    def put(self, key: int, value: int) -> None:
        print("HashMap.put:", key, "->", value)
        # Check if key exists, update if so
        i: int = 0
        found: int = 0
        while i < self.size:
            if self.keys[i] == key:
                print("Updating existing key")
                self.values[i] = value
                found = 1
                i = self.size  # Exit loop
            else:
                i = i + 1
        # Key not found, add new entry
        if found == 0:
            print("Adding new key")
            if self.size == 0:
                self.keys[0] = key
                self.values[0] = value
            else:
                self.keys.append(key)
                self.values.append(value)
            self.size = self.size + 1
        print("HashMap size:", self.size)

    def get(self, key: int) -> int:
        print("HashMap.get:", key)
        i: int = 0
        while i < self.size:
            if self.keys[i] == key:
                value: int = self.values[i]
                print("Found value:", value)
                return value
            i = i + 1
        print("Key not found")
        return 0  # Not found (use 0 as sentinel)

    def contains(self, key: int) -> int:
        print("HashMap.contains:", key)
        i: int = 0
        while i < self.size:
            if self.keys[i] == key:
                print("Key exists")
                return 1
            i = i + 1
        print("Key does not exist")
        return 0

    def get_size(self) -> int:
        print("Getting HashMap size")
        return self.size


def test_hashmap_basic() -> int:
    print("Test: HashMap basic operations")
    m: HashMap = HashMap()
    print("Inserting 3 entries")
    m.put(1, 100)
    m.put(2, 200)
    m.put(3, 300)
    print("Getting value for key 2")
    result: int = m.get(2)
    print("HashMap test complete")
    return result  # Expected: 200


def test_hashmap_update() -> int:
    m: HashMap = HashMap()
    m.put(1, 100)
    m.put(1, 999)  # Update existing key
    return m.get(1)  # Expected: 999


def test_hashmap_contains() -> int:
    m: HashMap = HashMap()
    m.put(5, 50)
    m.put(10, 100)
    result: int = m.contains(5) + m.contains(10) + m.contains(15)
    return result  # Expected: 2 (5 and 10 exist, 15 doesn't)
