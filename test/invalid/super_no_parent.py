# super() on class with no parent
class Base:
    def __init__(self) -> None:
        super().__init__()  # super() called but class has no parent
