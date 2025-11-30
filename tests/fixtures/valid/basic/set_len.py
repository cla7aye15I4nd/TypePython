def test() -> None:
    s: set[int] = {1, 2, 3}
    print(len(s))
    s.add(4)
    print(len(s))

test()
