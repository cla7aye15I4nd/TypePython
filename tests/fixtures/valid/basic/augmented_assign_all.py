def test() -> None:
    # Test all augmented assignment operators
    a: int = 10
    a -= 2
    print(a)  # 8

    b: int = 4
    b //= 2
    print(b)  # 2

    c: int = 7
    c %= 3
    print(c)  # 1

    d: int = 2
    d **= 3
    print(d)  # 8

    e: int = 5
    e |= 3
    print(e)  # 7

    f: int = 7
    f ^= 2
    print(f)  # 5

    g: int = 6
    g &= 3
    print(g)  # 2

    h: int = 2
    h <<= 2
    print(h)  # 8

    i: int = 16
    i >>= 2
    print(i)  # 4

test()
