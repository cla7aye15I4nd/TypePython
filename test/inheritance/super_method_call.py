# Tests for super() method calls (non __init__)

class Parent:
    value: int

    def __init__(self, v: int) -> None:
        self.value = v

    def compute(self, x: int) -> int:
        return self.value + x

    def double(self) -> int:
        return self.value * 2


class Child(Parent):
    multiplier: int

    def __init__(self, v: int, m: int) -> None:
        super().__init__(v)
        self.multiplier = m

    def compute(self, x: int) -> int:
        # Call parent's compute and add multiplier
        base: int = super().compute(x)
        return base * self.multiplier

    def triple(self) -> int:
        # Call parent's double and add value
        doubled: int = super().double()
        return doubled + self.value


def test_super_method_call() -> int:
    """Test calling super().method() on non-init method"""
    c: Child = Child(10, 3)
    # compute: (10 + 5) * 3 = 45
    return c.compute(5)

def test_super_paramless_method() -> int:
    """Test calling super() on paramless method"""
    c: Child = Child(7, 2)
    # triple: (7 * 2) + 7 = 21
    return c.triple()

def test_super_preserves_self() -> int:
    """Test that super() properly preserves self reference"""
    c: Child = Child(5, 4)
    # compute uses self.value from Parent via super
    result: int = c.compute(0)
    # (5 + 0) * 4 = 20
    return result
