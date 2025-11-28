# Division and integer division tests
def test_integer_division() -> int:
    div1: int = 100 // 3
    div2: int = 1000 // 7
    div3: int = 999 // 9
    div4: int = 12345 // 11
    return div1 + div2 + div3 + div4

def test_division_by_powers() -> int:
    pow2: int = 1024 // 2
    pow4: int = 1024 // 4
    pow8: int = 1024 // 8
    pow16: int = 1024 // 16
    return pow2 + pow4 + pow8 + pow16

def test_remainder_patterns() -> int:
    # Test patterns in modulo operations
    sum: int = 0
    i: int = 0
    while i < 20:
        sum = sum + (i % 7)
        i = i + 1
    return sum

result1: int = test_integer_division()
result2: int = test_division_by_powers()
result3: int = test_remainder_patterns()

print("Integer division:", result1)
print("Division by powers:", result2)
print("Remainder patterns:", result3)
