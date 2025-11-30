def test() -> None:
    t: bool = True
    f: bool = False

    # and
    print(t and t)
    print(t and f)
    print(f and t)
    print(f and f)

    # or
    print(t or t)
    print(t or f)
    print(f or t)
    print(f or f)

test()
