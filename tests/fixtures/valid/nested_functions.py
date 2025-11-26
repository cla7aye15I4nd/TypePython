# Multiple function definitions with nested calls

def add(a: int, b: int) -> int:
    return a + b

def multiply(x: int, y: int) -> int:
    return x * y

def compute(a: int, b: int, c: int) -> int:
    temp1: int = add(a, b)
    temp2: int = multiply(temp1, c)
    return temp2

# Function with complex logic
def is_even(n: int) -> bool:
    remainder: int = n % 2
    if remainder == 0:
        return True
    else:
        return False

# Using the functions
result: int = compute(5, 3, 2)
check: bool = is_even(42)
