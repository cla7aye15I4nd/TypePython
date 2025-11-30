def test() -> None:
    x: set[int] = {1, 2, 3}
    y: set[int] = x.copy()
    y.add(4)
    print(len(x))
    print(len(y))

test()
