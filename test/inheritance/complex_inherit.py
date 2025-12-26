# Test complex inheritance scenarios

# ========================================
# Multi-level inheritance with magic methods
# ========================================

class Level0:
    value: int

    def __init__(self, v: int) -> None:
        self.value = v

    def __len__(self) -> int:
        return self.value

    def get_level(self) -> int:
        return 0


class Level1(Level0):
    extra1: int

    def __init__(self, v: int, e1: int) -> None:
        super().__init__(v)
        self.extra1 = e1

    def get_level(self) -> int:
        return 1


class Level2(Level1):
    extra2: int

    def __init__(self, v: int, e1: int, e2: int) -> None:
        super().__init__(v, e1)
        self.extra2 = e2

    def get_level(self) -> int:
        return 2


class Level3(Level2):
    extra3: int

    def __init__(self, v: int, e1: int, e2: int, e3: int) -> None:
        super().__init__(v, e1, e2)
        self.extra3 = e3

    # Does not override get_level - uses Level2's


# ========================================
# Nested class with inheritance
# ========================================

class InnerBase:
    x: int
    y: int

    def __init__(self, a: int, b: int) -> None:
        self.x = a
        self.y = b

    def sum(self) -> int:
        return self.x + self.y


class InnerChild(InnerBase):
    z: int

    def __init__(self, a: int, b: int, c: int) -> None:
        super().__init__(a, b)
        self.z = c

    def sum(self) -> int:
        return self.x + self.y + self.z


class Outer:
    inner: InnerChild
    name: int

    def __init__(self, ic: InnerChild, n: int) -> None:
        self.inner = ic
        self.name = n

    def get_inner_sum(self) -> int:
        return self.inner.sum()


# ========================================
# List of inherited objects
# ========================================

class BaseItem:
    value: int

    def __init__(self, v: int) -> None:
        self.value = v

    def get_double(self) -> int:
        return self.value * 2


class DerivedItem(BaseItem):
    bonus: int

    def __init__(self, v: int, b: int) -> None:
        super().__init__(v)
        self.bonus = b

    def get_total(self) -> int:
        return self.value + self.bonus


# ========================================
# Test functions
# ========================================

def test_multilevel_len() -> int:
    """Test __len__ inherited through multiple levels"""
    l3: Level3 = Level3(42, 1, 2, 3)
    return len(l3)  # Expected: 42


def test_multilevel_fields() -> int:
    """Test all fields accessible in deep inheritance"""
    l3: Level3 = Level3(10, 20, 30, 40)
    return l3.value + l3.extra1 + l3.extra2 + l3.extra3  # Expected: 100


def test_multilevel_override_chain() -> int:
    """Test method override at different levels"""
    l0: Level0 = Level0(1)
    l1: Level1 = Level1(2, 10)
    l2: Level2 = Level2(3, 20, 30)
    l3: Level3 = Level3(4, 40, 50, 60)
    return l0.get_level() + l1.get_level() + l2.get_level() + l3.get_level()
    # Expected: 0 + 1 + 2 + 2 = 5 (l3 uses l2's get_level)


def test_nested_inheritance() -> int:
    """Test nested class with inheritance"""
    ic: InnerChild = InnerChild(10, 20, 30)
    o: Outer = Outer(ic, 99)
    return o.get_inner_sum()  # Expected: 60


def test_nested_chained_access() -> int:
    """Test chained access to inherited fields"""
    ic: InnerChild = InnerChild(5, 10, 15)
    o: Outer = Outer(ic, 1)
    return o.inner.x + o.inner.y + o.inner.z  # Expected: 30


def test_list_of_derived() -> int:
    """Test list containing derived class instances"""
    items: list[DerivedItem] = [
        DerivedItem(10, 1),
        DerivedItem(20, 2),
        DerivedItem(30, 3)
    ]
    total: int = 0
    i: int = 0
    while i < 3:
        total = total + items[i].get_total()
        i = i + 1
    return total  # Expected: 11 + 22 + 33 = 66


def test_derived_uses_parent_method() -> int:
    """Test derived class uses inherited method"""
    d: DerivedItem = DerivedItem(25, 5)
    return d.get_double()  # Expected: 50


def test_modify_inherited_field() -> int:
    """Test modifying inherited field"""
    d: DerivedItem = DerivedItem(10, 5)
    d.value = 100
    return d.get_double() + d.get_total()  # Expected: 200 + 105 = 305
