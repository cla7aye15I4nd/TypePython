# Test multi-level inheritance: A -> B -> C

class A:
    a: int

    def __init__(self, va: int) -> None:
        self.a = va

    def get_a(self) -> int:
        return self.a


class B(A):
    b: int

    def __init__(self, va: int, vb: int) -> None:
        super().__init__(va)
        self.b = vb

    def get_b(self) -> int:
        return self.b


class C(B):
    c: int

    def __init__(self, va: int, vb: int, vc: int) -> None:
        super().__init__(va, vb)
        self.c = vc

    def get_c(self) -> int:
        return self.c

    def get_sum(self) -> int:
        return self.a + self.b + self.c


def test_grandparent_method() -> int:
    """Test method from grandparent A"""
    obj: C = C(5, 10, 15)
    return obj.get_a()  # Expected: 5


def test_parent_method() -> int:
    """Test method from parent B"""
    obj: C = C(5, 10, 15)
    return obj.get_b()  # Expected: 10


def test_own_method() -> int:
    """Test own method"""
    obj: C = C(5, 10, 15)
    return obj.get_c()  # Expected: 15


def test_sum_method() -> int:
    """Test method accessing all inherited fields"""
    obj: C = C(1, 2, 3)
    return obj.get_sum()  # Expected: 6


def test_grandparent_field() -> int:
    """Test direct access to grandparent field"""
    obj: C = C(7, 8, 9)
    return obj.a  # Expected: 7


def test_parent_field() -> int:
    """Test direct access to parent field"""
    obj: C = C(7, 8, 9)
    return obj.b  # Expected: 8


def test_own_field() -> int:
    """Test direct access to own field"""
    obj: C = C(7, 8, 9)
    return obj.c  # Expected: 9


def test_all_fields_sum() -> int:
    """Test sum of all fields accessed directly"""
    obj: C = C(10, 20, 30)
    return obj.a + obj.b + obj.c  # Expected: 60
