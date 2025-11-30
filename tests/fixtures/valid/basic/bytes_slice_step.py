def test() -> None:
    b: bytes = b"hello world"
    # Slice with step
    s: bytes = b[::2]  # every 2nd byte
    print(s)

    s2: bytes = b[1::2]  # every 2nd byte starting at 1
    print(s2)

test()
