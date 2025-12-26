# super().method() where parent has no such method
class Base:
    def __init__(self) -> None:
        pass

class Child(Base):
    def __init__(self) -> None:
        super().nonexistent()  # Parent class has no method 'nonexistent'
