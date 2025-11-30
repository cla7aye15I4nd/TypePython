def test() -> None:
    x: dict[int, int] = {1: 10, 2: 20}
    y: dict[int, int] = x.copy()
    y[3] = 30
    print(len(x))
    print(len(y))

test()
