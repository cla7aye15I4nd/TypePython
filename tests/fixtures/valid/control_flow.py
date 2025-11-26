# Testing various control flow structures
def check_number(x: int) -> str:
    if x > 0:
        return "positive"
    elif x < 0:
        return "negative"
    else:
        return "zero"

# Loop with conditions
def count_down(start: int) -> int:
    counter: int = start
    while counter > 0:
        counter = counter - 1
    return counter

# Nested if statements
def classify(age: int, score: int) -> str:
    if age >= 18:
        if score >= 90:
            return "adult_excellent"
        else:
            return "adult_good"
    else:
        if score >= 90:
            return "young_excellent"
        else:
            return "young_good"

result1: str = check_number(5)
result2: int = count_down(10)
result3: str = classify(20, 95)
