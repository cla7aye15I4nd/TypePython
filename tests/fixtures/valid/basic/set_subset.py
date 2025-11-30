def test() -> None:
    s1: set[int] = {1, 2}
    s2: set[int] = {1, 2, 3}
    s3: set[int] = {4, 5}

    # issubset
    print(s1.issubset(s2))
    print(s2.issubset(s1))

    # issuperset
    print(s2.issuperset(s1))
    print(s1.issuperset(s2))

    # isdisjoint
    print(s1.isdisjoint(s3))
    print(s1.isdisjoint(s2))

test()
