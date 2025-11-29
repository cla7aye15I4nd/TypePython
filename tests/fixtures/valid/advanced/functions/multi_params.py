# Test function with multiple parameters
def add_five(a: int, b: int, c: int, d: int, e: int) -> int:
    return a + b + c + d + e

def concat_three(a: bytes, b: bytes, c: bytes) -> bytes:
    return a + b + c

print(add_five(1, 2, 3, 4, 5))
print(concat_three(b"Hello", b" ", b"World"))
