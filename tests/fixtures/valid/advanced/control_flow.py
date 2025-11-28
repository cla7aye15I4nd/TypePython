# Testing various control flow structures
def check_number(x: int) -> bytes:
    if x > 0:
        return b"positive"
    elif x < 0:
        return b"negative"
    else:
        return b"zero"

# Loop with conditions
def count_down(start: int) -> int:
    counter: int = start
    while counter > 0:
        counter = counter - 1
    return counter

# Nested if statements
def classify(age: int, score: int) -> bytes:
    if age >= 18:
        if score >= 90:
            return b"adult_excellent"
        else:
            return b"adult_good"
    else:
        if score >= 90:
            return b"young_excellent"
        else:
            return b"young_good"

result1: bytes = check_number(5)
result2: int = count_down(10)
result3: bytes = classify(20, 95)

print(b"Check number 5:", result1)
print(b"Count down from 10:", result2)
print(b"Classify (20, 95):", result3)
