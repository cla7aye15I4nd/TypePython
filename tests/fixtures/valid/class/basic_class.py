# Test basic class with fields and methods
class Point:
    x: int
    y: int

    def __init__(self, x: int, y: int) -> None:
        self.x = x
        self.y = y

    def get_x(self) -> int:
        return self.x

    def get_y(self) -> int:
        return self.y

    def get_sum(self) -> int:
        return self.x + self.y

p: Point = Point(10, 20)
print(b"x:", p.get_x())
print(b"y:", p.get_y())
print(b"sum:", p.get_sum())
