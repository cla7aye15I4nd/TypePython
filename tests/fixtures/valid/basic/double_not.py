def test() -> None:
    x: bool = True
    y: bool = not not x
    print(y)

    z: bool = not not not False
    print(z)

test()
