# Method call with wrong argument count
class Foo:
    def bar(self, x: int) -> int:
        return x

def main() -> None:
    f: Foo = Foo()
    r: int = f.bar()  # Missing argument
