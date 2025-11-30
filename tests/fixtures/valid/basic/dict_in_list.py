def test() -> None:
    d: dict[int, int] = {1: 10}
    lst: list[int] = [1, 2, 3]

    # Dict in list always returns False (can't have dicts in int list)
    print(d in lst)
    print(d not in lst)

test()
