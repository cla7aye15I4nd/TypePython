# iter() with wrong argument count
def main() -> None:
    x: list[int] = [1, 2, 3]
    y = iter(x, x)  # iter() takes exactly one argument
