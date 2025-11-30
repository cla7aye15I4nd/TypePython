def test() -> None:
    # int and int -> returns int (Python semantics)
    x: int = 5 and 3
    print(x)  # 3

    y: int = 0 and 5
    print(y)  # 0

    # int or int -> returns int (Python semantics)
    z: int = 5 or 3
    print(z)  # 5

    w: int = 0 or 10
    print(w)  # 10

test()
