# Accessing non-existent field or method
class Point:
    x: int
    y: int

    def __init__(self, x: int, y: int) -> None:
        self.x = x
        self.y = y

def main() -> None:
    p: Point = Point(1, 2)
    z: int = p.z
    print(z)
