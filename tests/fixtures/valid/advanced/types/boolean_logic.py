# Complex boolean logic and expressions
def test_and_operations() -> int:
    count: int = 0

    if True and True:
        count = count + 1
    if True and False:
        count = count + 1
    if False and True:
        count = count + 1
    if False and False:
        count = count + 1

    return count

def test_or_operations() -> int:
    count: int = 0

    if True or True:
        count = count + 1
    if True or False:
        count = count + 1
    if False or True:
        count = count + 1
    if False or False:
        count = count + 1

    return count

def test_not_operations() -> int:
    count: int = 0

    if not False:
        count = count + 1
    if not True:
        count = count + 1

    return count

def test_complex_boolean() -> int:
    a: bool = True
    b: bool = False
    c: bool = True

    result: int = 0
    if (a and b) or c:
        result = result + 1
    if a and (b or c):
        result = result + 1
    if not (a and b):
        result = result + 1
    if not a or not b:
        result = result + 1

    return result

result1: int = test_and_operations()
result2: int = test_or_operations()
result3: int = test_not_operations()
result4: int = test_complex_boolean()

print("AND operations:", result1)
print("OR operations:", result2)
print("NOT operations:", result3)
print("Complex boolean:", result4)
