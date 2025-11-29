# Cannot right shift Set[int] and Bytes
x: set[int] = {1, 2, 3} >> b"hello"
