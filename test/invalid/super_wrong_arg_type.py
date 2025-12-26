# super().method() with wrong argument type
class Base:
    def foo(self, x: int) -> None:
        pass

class Child(Base):
    def bar(self) -> None:
        super().foo(True)  # Argument type mismatch
