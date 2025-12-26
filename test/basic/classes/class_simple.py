class Point:
    x: int
    y: int

    def __init__(self, a: int, b: int) -> None:
        print("Point.__init__ called")
        self.x = a
        self.y = b
        print("Point created with x =", a, "y =", b)

    def __str__(self) -> str:
        return "Point(x, y)"

    def get_x(self) -> int:
        print("Getting x value")
        return self.x

    def get_sum(self) -> int:
        print("Calculating sum of x and y")
        total: int = self.x + self.y
        print("Sum is", total)
        return total
