def test() -> None:
    d1: dict[str, int] = {"a": 1, "b": 2}
    d2: dict[str, int] = {"c": 3, "d": 4}
    d3: dict[str, int] = d1 | d2
    print(len(d3))

test()
