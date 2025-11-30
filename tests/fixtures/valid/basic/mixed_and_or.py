def test() -> None:
    # Mixed types with and/or -> returns bool (different types)
    # wrap with bool() for Python compatibility
    x: bool = bool(True and 5)
    print(x)

    y: bool = bool(False or 10)
    print(y)

    z: bool = bool(True and 3.14)
    print(z)

    w: bool = bool(0 or 2.71)
    print(w)

test()
