# Tower of Hanoi - count moves
def hanoi_moves(n: int) -> int:
    if n == 1:
        return 1
    else:
        return 2 * hanoi_moves(n - 1) + 1

def test_hanoi() -> int:
    h1: int = hanoi_moves(1)
    h2: int = hanoi_moves(2)
    h3: int = hanoi_moves(3)
    h4: int = hanoi_moves(4)
    h5: int = hanoi_moves(5)

    return h1 + h2 + h3 + h4 + h5

result: int = test_hanoi()
print("Hanoi moves sum:", result)
print("Hanoi(1):", hanoi_moves(1))
print("Hanoi(3):", hanoi_moves(3))
print("Hanoi(5):", hanoi_moves(5))
