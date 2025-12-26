# next() with wrong argument count
def main() -> None:
    x: list[int] = [1, 2, 3]
    it = iter(x)
    y = next(it, it)  # next() takes exactly one argument
