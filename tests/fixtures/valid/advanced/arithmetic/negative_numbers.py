# Negative number arithmetic
def test_negative_arithmetic() -> int:
    neg1: int = -10 + 5
    neg2: int = -20 - -15
    neg3: int = -7 * 3
    neg4: int = -100 // -3
    return neg1 + neg2 + neg3 + neg4

def test_negative_combinations() -> int:
    a: int = -((10 + 5) - 20)
    b: int = -(100 - 150)
    c: int = -(-42)
    return a + b + c

def test_mixed_signs() -> int:
    result: int = 10 - -5 + -3 - -7
    return result

result1: int = test_negative_arithmetic()
result2: int = test_negative_combinations()
result3: int = test_mixed_signs()

print(b"Negative arithmetic:", result1)
print(b"Negative combinations:", result2)
print(b"Mixed signs:", result3)
