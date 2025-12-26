# Magic methods test: __len__, __getitem__, __setitem__

# ============ __len__ tests ============

def test_list_int_len() -> int:
    nums: list[int] = [10, 20, 30, 40, 50]
    return len(nums)  # Expected: 5

def test_list_str_len() -> int:
    words: list[str] = ["hello", "world", "test"]
    return len(words)  # Expected: 3

def test_str_len() -> int:
    s: str = "hello"
    return len(s)  # Expected: 5

def test_str_empty_len() -> int:
    s: str = ""
    return len(s)  # Expected: 0

# ============ __getitem__ tests ============

def test_list_getitem_first() -> int:
    nums: list[int] = [100, 200, 300]
    return nums[0]  # Expected: 100

def test_list_getitem_middle() -> int:
    nums: list[int] = [10, 20, 30, 40, 50]
    return nums[2]  # Expected: 30

def test_list_getitem_last() -> int:
    nums: list[int] = [5, 10, 15, 20, 25]
    return nums[4]  # Expected: 25

def test_bytes_getitem_first() -> int:
    b: bytes = b"hello"
    return b[0]  # Expected: 104 (ASCII 'h')

def test_bytes_getitem_last() -> int:
    b: bytes = b"hello"
    return b[4]  # Expected: 111 (ASCII 'o')

def test_bytearray_getitem_first() -> int:
    ba: bytearray = bytearray(b"world")
    return ba[0]  # Expected: 119 (ASCII 'w')

def test_bytearray_getitem_last() -> int:
    ba: bytearray = bytearray(b"world")
    return ba[4]  # Expected: 100 (ASCII 'd')

# ============ __setitem__ tests ============

def test_list_setitem_first() -> int:
    nums: list[int] = [1, 2, 3]
    nums[0] = 100
    return nums[0]  # Expected: 100

def test_list_setitem_middle() -> int:
    nums: list[int] = [10, 20, 30, 40, 50]
    nums[2] = 999
    return nums[2]  # Expected: 999

def test_list_setitem_last() -> int:
    nums: list[int] = [5, 10, 15]
    nums[2] = 42
    return nums[2]  # Expected: 42

def test_bytearray_setitem_first() -> int:
    ba: bytearray = bytearray(b"abc")
    ba[0] = 65  # 'A'
    return ba[0]  # Expected: 65

def test_bytearray_setitem_middle() -> int:
    ba: bytearray = bytearray(b"hello")
    ba[2] = 88  # 'X'
    return ba[2]  # Expected: 88

def test_bytearray_setitem_last() -> int:
    ba: bytearray = bytearray(b"test")
    ba[3] = 90  # 'Z'
    return ba[3]  # Expected: 90

# ============ Combined tests ============

def test_list_len_after_setitem() -> int:
    nums: list[int] = [1, 2, 3, 4, 5]
    nums[0] = 100
    nums[4] = 500
    return len(nums)  # Expected: 5 (length unchanged)

def test_list_sum_after_setitem() -> int:
    nums: list[int] = [10, 20, 30]
    nums[1] = 50
    return nums[0] + nums[1] + nums[2]  # Expected: 10 + 50 + 30 = 90

def test_bytearray_sum_after_setitem() -> int:
    ba: bytearray = bytearray(b"ab")
    ba[0] = 10
    ba[1] = 20
    return ba[0] + ba[1]  # Expected: 30

# ============ Custom class tests ============

class Container:
    data: list[int]
    size: int

    def __init__(self, a: int, b: int, c: int) -> None:
        self.data = [a, b, c]
        self.size = 3

    def __len__(self) -> int:
        print("Calculating length")
        return self.size

    def __getitem__(self, index: int) -> int:
        print("Getting item at index", index)
        return self.data[index]

    def __setitem__(self, index: int, value: int) -> None:
        print("Setting item at index", index, "to", value)
        self.data[index] = value

def test_custom_len() -> int:
    c: Container = Container(10, 20, 30)
    return len(c)  # Expected: 3

def test_custom_getitem_first() -> int:
    c: Container = Container(100, 200, 300)
    return c[0]  # Expected: 100

def test_custom_getitem_last() -> int:
    c: Container = Container(5, 10, 15)
    return c[2]  # Expected: 15

def test_custom_setitem() -> int:
    c: Container = Container(1, 2, 3)
    c[1] = 999
    return c[1]  # Expected: 999

def test_custom_setitem_and_sum() -> int:
    c: Container = Container(10, 20, 30)
    c[0] = 100
    c[2] = 300
    return c[0] + c[1] + c[2]  # Expected: 100 + 20 + 300 = 420

class StrContainer:
    items: list[str]
    count: int

    def __init__(self, a: str, b: str) -> None:
        self.items = [a, b]
        self.count = 2

    def __len__(self) -> int:
        return self.count

    def __getitem__(self, index: int) -> str:
        return self.items[index]

    def __setitem__(self, index: int, value: str) -> None:
        self.items[index] = value

def test_custom_str_len() -> int:
    sc: StrContainer = StrContainer("hello", "world")
    return len(sc)  # Expected: 2

def test_custom_str_getitem() -> int:
    sc: StrContainer = StrContainer("hello", "world")
    s: str = sc[0]
    return len(s)  # Expected: 5 (len of "hello")

def test_custom_str_setitem() -> int:
    sc: StrContainer = StrContainer("hi", "bye")
    sc[0] = "hello"
    s: str = sc[0]
    return len(s)  # Expected: 5 (len of "hello")
