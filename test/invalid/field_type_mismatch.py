# Field assignment type mismatch
class Point:
    x: int
    y: int

    def __init__(self, a: int, b: int) -> None:
        self.x = a
        self.y = b

def main() -> None:
    p: Point = Point(1, 2)
    p.x = True  # Cannot assign bool to int field
