# Tests for __str__ and __repr__ methods

# Test 1: Class with __str__ method only
class Point:
    x: int
    y: int

    def __init__(self, x: int, y: int) -> None:
        self.x = x
        self.y = y

    def __str__(self) -> str:
        return "Point(x, y)"

# Test 2: Class with __repr__ method only
class Vector:
    x: int
    y: int

    def __init__(self, x: int, y: int) -> None:
        self.x = x
        self.y = y

    def __repr__(self) -> str:
        return "Vector[x, y]"

# Test 3: Class with both __str__ and __repr__ methods
# __str__ should be used when printing
class Rectangle:
    width: int
    height: int

    def __init__(self, w: int, h: int) -> None:
        self.width = w
        self.height = h

    def __str__(self) -> str:
        return "Rectangle(w x h)"

    def __repr__(self) -> str:
        return "Rect[w, h]"

# Test 4: Class with __str__ that prints inside the method
class Person:
    name: str
    age: int

    def __init__(self, name: str, age: int) -> None:
        self.name = name
        self.age = age

    def __str__(self) -> str:
        print("Inside __str__ method")
        return "Person(name, age)"

# Test 5: Class with __repr__ that prints inside the method
class Book:
    title: str
    pages: int

    def __init__(self, title: str, pages: int) -> None:
        self.title = title
        self.pages = pages

    def __repr__(self) -> str:
        print("Inside __repr__ method")
        return "Book[title, pages]"

# Test 6: Nested class with __str__
class Outer:
    value: int

    def __init__(self, v: int) -> None:
        self.value = v

    def __str__(self) -> str:
        return "Outer(value)"

class Container:
    inner: Outer

    def __init__(self, o: Outer) -> None:
        self.inner = o

    def __str__(self) -> str:
        print("Printing inner from Container.__str__:")
        print(self.inner)
        return "Container(inner)"


def test_str_only() -> int:
    # Test printing class with __str__ method
    p: Point = Point(10, 20)
    print(p)
    return 1

def test_repr_only() -> int:
    # Test printing class with __repr__ method
    v: Vector = Vector(3, 4)
    print(v)
    return 1

def test_both_str_and_repr() -> int:
    # When both exist, __str__ should be used
    r: Rectangle = Rectangle(100, 50)
    print(r)
    return 1

def test_str_with_internal_print() -> int:
    # Test that prints inside __str__ work
    p: Person = Person("Alice", 30)
    print("Before printing person:")
    print(p)
    print("After printing person")
    return 1

def test_repr_with_internal_print() -> int:
    # Test that prints inside __repr__ work
    b: Book = Book("Python Guide", 500)
    print("Before printing book:")
    print(b)
    print("After printing book")
    return 1

def test_nested_with_str() -> int:
    # Test nested class printing with __str__
    o: Outer = Outer(42)
    c: Container = Container(o)
    print("Printing container:")
    print(c)
    return 1

def test_multiple_instances() -> int:
    # Test multiple instances
    p1: Point = Point(1, 2)
    p2: Point = Point(3, 4)
    print(p1)
    print(p2)
    return 2

def test_str_in_expression() -> int:
    # Test using instance in different contexts
    p: Point = Point(5, 10)
    print(p)
    # Can still access fields normally
    x_val: int = p.x
    print(x_val)
    return 1
