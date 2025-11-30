def returns_none() -> None:
    print(1)
    return

def test() -> None:
    returns_none()
    print(2)

test()
