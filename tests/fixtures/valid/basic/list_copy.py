def test() -> None:
    x: list[int] = [1, 2, 3]
    y: list[int] = x.copy()
    y.append(4)
    print(len(x))
    print(len(y))

test()
