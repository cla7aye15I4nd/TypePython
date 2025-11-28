# Complex arithmetic expressions with operator precedence
def test_precedence() -> int:
    # PEMDAS: Parentheses, Exponents, Multiplication/Division, Addition/Subtraction
    result1: int = 2 + 3 * 4 - 6 // 2
    result2: int = (2 + 3) * (4 - 6 // 2)
    result3: int = 10 - 5 - 3
    result4: int = 100 // 10 // 2

    total: int = result1 + result2 + result3 + result4
    return total

def test_nested_operations() -> int:
    a: int = ((10 + 5) * 2 - 3) * ((8 - 2) + 4)
    b: int = (100 - ((20 + 10) * 2)) + 15
    return a + b

def test_modulo_operations() -> int:
    mod1: int = 17 % 5
    mod2: int = 100 % 13
    mod3: int = 1000 % 7
    mod4: int = 999 % 111
    return mod1 + mod2 + mod3 + mod4

result1: int = test_precedence()
result2: int = test_nested_operations()
result3: int = test_modulo_operations()

print("Precedence test:", result1)
print("Nested operations:", result2)
print("Modulo operations:", result3)
print("Total:", result1 + result2 + result3)
