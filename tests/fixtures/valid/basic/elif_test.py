# Test elif clauses
def check_value(x: int) -> int:
    if x < 0:
        return -1
    elif x == 0:
        return 0
    elif x < 10:
        return 1
    elif x < 100:
        return 2
    else:
        return 3

print(check_value(-5))
print(check_value(0))
print(check_value(5))
print(check_value(50))
print(check_value(200))

# Test elif without else
def grade(score: int) -> int:
    if score >= 90:
        return 5
    elif score >= 80:
        return 4
    elif score >= 70:
        return 3
    elif score >= 60:
        return 2
    return 1

print(grade(95))
print(grade(85))
print(grade(75))
print(grade(65))
print(grade(50))
