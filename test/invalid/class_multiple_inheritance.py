class Base1:
    def method1(self) -> int:
        return 1

class Base2:
    def method2(self) -> int:
        return 2

class Child(Base1, Base2):
    def method3(self) -> int:
        return 3

def main() -> None:
    c = Child()
    c.method1()
    c.method2()
    c.method3()
