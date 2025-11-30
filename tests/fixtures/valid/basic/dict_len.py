def test() -> None:
    d: dict[int, int] = {1: 10, 2: 20}
    print(len(d))
    d[3] = 30
    print(len(d))

test()
