# Complex types test cases: class in class, list[ClassName], chained access

class Point:
    x: int
    y: int

    def __init__(self, a: int, b: int) -> None:
        print("Creating Point with", a, b)
        self.x = a
        self.y = b

    def __str__(self) -> str:
        return "Point(x, y)"

    def __repr__(self) -> str:
        return "Point[x, y]"

    def get_sum(self) -> int:
        print("Point.get_sum called")
        result: int = self.x + self.y
        print("Sum =", result)
        return result


class Rectangle:
    corner: Point
    width: int
    height: int

    def __init__(self, c: Point, w: int, h: int) -> None:
        print("Creating Rectangle")
        self.corner = c
        self.width = w
        self.height = h
        print("Rectangle created with width", w, "height", h)

    def __str__(self) -> str:
        return "Rectangle(w, h)"

    def get_corner_x(self) -> int:
        print("Getting corner x")
        return self.corner.x

    def get_corner_sum(self) -> int:
        print("Getting corner sum")
        return self.corner.get_sum()


# Test 1: Class instance field access (r.corner.x)
def test_class_in_class() -> int:
    print("Test: class in class")
    p: Point = Point(5, 10)
    print("Created point, now creating rectangle")
    r: Rectangle = Rectangle(p, 100, 50)
    print("Rectangle created, accessing corner.x")
    result: int = r.corner.x
    print("Result:", result)
    return result


# Test 2: Chained attribute assignment (r.corner.x = 42)
def test_chained_assign() -> int:
    print("Test: chained assign")
    p: Point = Point(0, 0)
    r: Rectangle = Rectangle(p, 10, 20)
    print("Setting corner.x to 42")
    r.corner.x = 42
    print("corner.x is now", r.corner.x)
    return r.corner.x


# Test 3: Method call on nested object (r.corner.get_sum())
def test_nested_method() -> int:
    print("Test: nested method")
    p: Point = Point(3, 7)
    r: Rectangle = Rectangle(p, 50, 50)
    print("Calling get_corner_sum")
    result: int = r.get_corner_sum()
    return result


# Test 4: Multiple chained assignments
def test_multiple_chained() -> int:
    print("Test: multiple chained")
    p: Point = Point(1, 1)
    r: Rectangle = Rectangle(p, 5, 5)
    print("Setting multiple fields")
    r.corner.x = 10
    print("Set x to 10")
    r.corner.y = 20
    print("Set y to 20")
    sum_val: int = r.corner.x + r.corner.y
    print("Sum:", sum_val)
    return sum_val


# Test 5: List subscript assignment (points[0] = Point(...))
def test_list_set() -> int:
    print("Test: list set")
    p1: Point = Point(1, 1)
    points: list[Point] = [p1]
    print("Created list with first point")
    p2: Point = Point(99, 99)
    print("Replacing first point")
    points[0] = p2
    print("First point is now", points[0])
    return points[0].x


# Test 6: List of class instances with element access
def test_list_of_class() -> int:
    print("Test: list of class")
    p1: Point = Point(10, 20)
    p2: Point = Point(30, 40)
    points: list[Point] = [p1, p2]
    print("Created list with 2 points")
    result: int = points[0].x + points[1].y
    print("Result:", result)
    return result


# Test 7: Modify element in list of classes
def test_list_element_modify() -> int:
    print("Test: list element modify")
    p: Point = Point(5, 5)
    points: list[Point] = [p]
    print("Modifying first point x to 100")
    points[0].x = 100
    print("Modified point:", points[0])
    return points[0].x


# Test 8: Deep nesting - Rectangle with Point, access corner and modify
def test_deep_nesting() -> int:
    print("Test: deep nesting")
    p: Point = Point(1, 2)
    r: Rectangle = Rectangle(p, 10, 20)
    original: int = r.corner.x
    print("Original x:", original)
    r.corner.x = r.corner.x + r.corner.y
    modified: int = r.corner.x
    print("Modified x:", modified)
    result: int = original + modified
    print("Total:", result)
    return result


def main() -> int:
    result: int = 0
    result = result + test_class_in_class()
    result = result + test_chained_assign()
    result = result + test_nested_method()
    result = result + test_multiple_chained()
    result = result + test_list_set()
    result = result + test_list_of_class()
    result = result + test_list_element_modify()
    result = result + test_deep_nesting()
    return result
