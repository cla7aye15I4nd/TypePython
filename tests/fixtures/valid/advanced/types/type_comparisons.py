# Type comparison edge cases
def compare_integers() -> int:
    count: int = 0

    if 10 == 10:
        count = count + 1
    if 10 != 20:
        count = count + 1
    if 10 < 20:
        count = count + 1
    if 20 > 10:
        count = count + 1
    if 10 <= 10:
        count = count + 1
    if 10 >= 10:
        count = count + 1

    return count

def compare_floats() -> int:
    count: int = 0

    if 10.5 > 10.0:
        count = count + 1
    if 10.0 < 10.5:
        count = count + 1
    if 10.5 == 10.5:
        count = count + 1
    if 10.5 != 10.0:
        count = count + 1

    return count

def compare_booleans() -> int:
    count: int = 0

    if True == True:
        count = count + 1
    if False == False:
        count = count + 1
    if True != False:
        count = count + 1

    return count

def chained_comparisons(x: int) -> int:
    count: int = 0

    if x > 5:
        if x < 15:
            count = count + 1

    if x >= 10:
        if x <= 10:
            count = count + 1

    return count

result1: int = compare_integers()
result2: int = compare_floats()
result3: int = compare_booleans()
result4: int = chained_comparisons(10)

print("Compare integers:", result1)
print("Compare floats:", result2)
print("Compare booleans:", result3)
print("Chained comparisons:", result4)
