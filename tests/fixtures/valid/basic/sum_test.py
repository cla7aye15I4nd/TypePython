def test() -> None:
    x: list[int] = [1, 2, 3, 4, 5]
    print(sum(x))

    # Sum with start value
    print(sum(x, 10))

test()
