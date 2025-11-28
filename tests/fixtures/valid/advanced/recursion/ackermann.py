# Ackermann function (highly recursive)
def ackermann(m: int, n: int) -> int:
    if m == 0:
        return n + 1
    else:
        if n == 0:
            return ackermann(m - 1, 1)
        else:
            return ackermann(m - 1, ackermann(m, n - 1))

def test_ackermann() -> int:
    a00: int = ackermann(0, 0)
    a01: int = ackermann(0, 5)
    a10: int = ackermann(1, 0)
    a11: int = ackermann(1, 5)
    a20: int = ackermann(2, 0)
    a22: int = ackermann(2, 2)

    return a00 + a01 + a10 + a11 + a20 + a22

result: int = test_ackermann()
print(b"Ackermann test result:", result)
print(b"A(3, 2):", ackermann(3, 2))
