# Simple inheritance test

class Animal:
    age: int

    def __init__(self, a: int) -> None:
        self.age = a

    def speak(self) -> int:
        return 0


class Dog(Animal):
    legs: int

    def __init__(self, a: int, l: int) -> None:
        super().__init__(a)
        self.legs = l

    def speak(self) -> int:
        return 1


def test_simple_inherit() -> int:
    """Test basic field and method inheritance"""
    dog: Dog = Dog(3, 4)
    return dog.age + dog.legs + dog.speak()  # Expected: 3 + 4 + 1 = 8


def test_inherited_field() -> int:
    """Test inherited field access"""
    dog: Dog = Dog(5, 4)
    return dog.age  # Expected: 5


def test_own_field() -> int:
    """Test own field access"""
    dog: Dog = Dog(3, 6)
    return dog.legs  # Expected: 6


def test_overridden_method() -> int:
    """Test method override"""
    dog: Dog = Dog(1, 2)
    return dog.speak()  # Expected: 1
