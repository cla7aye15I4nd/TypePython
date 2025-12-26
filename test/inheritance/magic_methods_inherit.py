# Test inheritance with magic methods (__len__, __getitem__, __setitem__)

# Base class with magic methods
class BaseContainer:
    data: list[int]
    size: int

    def __init__(self, a: int, b: int, c: int) -> None:
        self.data = [a, b, c]
        self.size = 3

    def __len__(self) -> int:
        return self.size

    def __getitem__(self, index: int) -> int:
        return self.data[index]

    def __setitem__(self, index: int, value: int) -> None:
        self.data[index] = value

    def get_sum(self) -> int:
        total: int = 0
        i: int = 0
        while i < self.size:
            total = total + self.data[i]
            i = i + 1
        return total


# Child class that inherits magic methods
class ExtendedContainer(BaseContainer):
    name: int  # Using int to represent name ID

    def __init__(self, a: int, b: int, c: int, n: int) -> None:
        super().__init__(a, b, c)
        self.name = n

    # Inherits __len__, __getitem__, __setitem__, get_sum from BaseContainer

    def get_name(self) -> int:
        return self.name


# Child class that overrides some magic methods
class CustomContainer(BaseContainer):
    multiplier: int

    def __init__(self, a: int, b: int, c: int, m: int) -> None:
        super().__init__(a, b, c)
        self.multiplier = m

    # Override __getitem__ to multiply values
    def __getitem__(self, index: int) -> int:
        return self.data[index] * self.multiplier

    # Inherits __len__ and __setitem__ from BaseContainer


# Test functions for inherited magic methods
def test_inherit_len() -> int:
    """Test __len__ is inherited"""
    c: ExtendedContainer = ExtendedContainer(10, 20, 30, 1)
    return len(c)  # Expected: 3


def test_inherit_getitem_first() -> int:
    """Test __getitem__ is inherited - first element"""
    c: ExtendedContainer = ExtendedContainer(100, 200, 300, 1)
    return c[0]  # Expected: 100


def test_inherit_getitem_last() -> int:
    """Test __getitem__ is inherited - last element"""
    c: ExtendedContainer = ExtendedContainer(100, 200, 300, 1)
    return c[2]  # Expected: 300


def test_inherit_setitem() -> int:
    """Test __setitem__ is inherited"""
    c: ExtendedContainer = ExtendedContainer(10, 20, 30, 1)
    c[1] = 999
    return c[1]  # Expected: 999


def test_inherit_setitem_sum() -> int:
    """Test inherited setitem affects sum"""
    c: ExtendedContainer = ExtendedContainer(10, 20, 30, 1)
    c[0] = 100
    return c.get_sum()  # Expected: 100 + 20 + 30 = 150


def test_inherit_own_field() -> int:
    """Test child class has its own field"""
    c: ExtendedContainer = ExtendedContainer(10, 20, 30, 42)
    return c.get_name()  # Expected: 42


def test_inherit_combined() -> int:
    """Test combination of inherited and own methods"""
    c: ExtendedContainer = ExtendedContainer(5, 10, 15, 99)
    c[0] = 100  # Use inherited __setitem__
    result: int = c.get_sum() + c.get_name()  # 100+10+15 + 99 = 224
    return result  # Expected: 224


# Test functions for overridden magic methods
def test_override_getitem() -> int:
    """Test __getitem__ is overridden"""
    c: CustomContainer = CustomContainer(10, 20, 30, 2)
    return c[0]  # Expected: 10 * 2 = 20


def test_override_getitem_all() -> int:
    """Test all elements through overridden __getitem__"""
    c: CustomContainer = CustomContainer(5, 10, 15, 3)
    return c[0] + c[1] + c[2]  # Expected: 15 + 30 + 45 = 90


def test_override_inherit_len() -> int:
    """Test __len__ is still inherited when __getitem__ is overridden"""
    c: CustomContainer = CustomContainer(1, 2, 3, 10)
    return len(c)  # Expected: 3


def test_override_inherit_setitem() -> int:
    """Test __setitem__ is still inherited when __getitem__ is overridden"""
    c: CustomContainer = CustomContainer(10, 20, 30, 2)
    c[1] = 100  # Use inherited __setitem__
    return c[1]  # Expected: 100 * 2 = 200


def test_override_setitem_then_get() -> int:
    """Test setitem then getitem with override"""
    c: CustomContainer = CustomContainer(1, 2, 3, 5)
    c[0] = 10  # Set to 10
    c[1] = 20  # Set to 20
    c[2] = 30  # Set to 30
    return c[0] + c[1] + c[2]  # Expected: 50 + 100 + 150 = 300
