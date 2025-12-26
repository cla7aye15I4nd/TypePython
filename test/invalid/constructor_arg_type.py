# Constructor with wrong argument type
class Point:
    x: int
    y: int

    def __init__(self, x: int, y: int) -> None:
        self.x = x
        self.y = y

def main() -> None:
    p: Point = Point("hello", 2)
    print(p.x)
