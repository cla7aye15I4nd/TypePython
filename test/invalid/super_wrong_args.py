# super().method() with wrong argument count
class Base:
    def foo(self, x: int) -> None:
        pass

class Child(Base):
    def bar(self) -> None:
        super().foo()  # super().foo() expects 1 arguments, got 0
