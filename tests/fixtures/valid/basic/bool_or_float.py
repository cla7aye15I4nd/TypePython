def test() -> None:
    # bool or float: different types -> returns bool
    # wrap with bool() for Python compatibility
    x: bool = bool(False or 3.14)
    print(x)

    y: bool = bool(True or 2.71)
    print(y)

test()
