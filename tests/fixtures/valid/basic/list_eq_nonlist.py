def test() -> None:
    # list == non-list should return False
    lst: list[int] = [1, 2, 3]
    x: int = 5
    result: bool = lst == x
    print(result)

test()
