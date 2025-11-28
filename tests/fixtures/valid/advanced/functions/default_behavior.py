# Functions with default-like behavior
def greet(name: bytes, formal: bool) -> None:
    if formal:
        print(b"Good day,", name)
    else:
        print(b"Hey,", name)

def compute_discount(price: int, is_member: bool) -> int:
    discount: int = 0

    if is_member:
        discount = price // 10
    else:
        discount = price // 20

    return price - discount

def format_number(n: int, use_hex: bool) -> None:
    if use_hex:
        # Simulate hex output
        print(b"Hex mode:", n)
    else:
        print(b"Decimal:", n)

greet(b"Alice", True)
greet(b"Bob", False)

result1: int = compute_discount(100, True)
result2: int = compute_discount(100, False)

print(b"Member discount:", result1)
print(b"Regular discount:", result2)

format_number(255, True)
format_number(255, False)
