# GCD using Euclidean algorithm (recursive)
def gcd(a: int, b: int) -> int:
    if b == 0:
        return a
    else:
        return gcd(b, a % b)

def lcm(a: int, b: int) -> int:
    gcd_val: int = gcd(a, b)
    return (a * b) // gcd_val

def test_multiple_gcds() -> int:
    g1: int = gcd(48, 18)
    g2: int = gcd(100, 35)
    g3: int = gcd(54, 24)
    return g1 + g2 + g3

def test_multiple_lcms() -> int:
    l1: int = lcm(4, 6)
    l2: int = lcm(3, 7)
    l3: int = lcm(12, 18)
    return l1 + l2 + l3

result1: int = test_multiple_gcds()
result2: int = test_multiple_lcms()

print(b"GCD tests sum:", result1)
print(b"LCM tests sum:", result2)
print(b"GCD(48, 18):", gcd(48, 18))
print(b"LCM(12, 18):", lcm(12, 18))
