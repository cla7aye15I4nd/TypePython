# Test method inheritance without override

class Base:
    value: int

    def __init__(self, v: int) -> None:
        self.value = v

    def get_value(self) -> int:
        return self.value

    def double(self) -> int:
        return self.value * 2


class Child(Base):
    extra: int

    def __init__(self, v: int, e: int) -> None:
        super().__init__(v)
        self.extra = e

    # Inherits get_value and double from Base


def test_inherit_method() -> int:
    """Test inherited method"""
    c: Child = Child(10, 5)
    return c.get_value()  # Expected: 10


def test_inherit_method_computed() -> int:
    """Test inherited computed method"""
    c: Child = Child(15, 5)
    return c.double()  # Expected: 30


def test_own_field_in_child() -> int:
    """Test child's own field"""
    c: Child = Child(10, 7)
    return c.extra  # Expected: 7


def test_combined() -> int:
    """Test inherited methods with own field"""
    c: Child = Child(10, 5)
    return c.get_value() + c.double() + c.extra
    # Expected: 10 + 20 + 5 = 35
