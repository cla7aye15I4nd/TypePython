# Functions with default-like behavior
def greet(name: str, formal: bool) -> None:
    if formal:
        print("Good day,", name)
    else:
        print("Hey,", name)

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
        print("Hex mode:", n)
    else:
        print("Decimal:", n)

greet("Alice", True)
greet("Bob", False)

result1: int = compute_discount(100, True)
result2: int = compute_discount(100, False)

print("Member discount:", result1)
print("Regular discount:", result2)

format_number(255, True)
format_number(255, False)
