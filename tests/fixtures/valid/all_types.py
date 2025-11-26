# Demonstrate all type system features

# Integer type
age: int = 25
count: int = 100

# Float type
temperature: float = 98.6
price: float = 19.99

# Boolean type
is_valid: bool = True
is_complete: bool = False

# String type
name: str = "TypePython"
message: str = "Hello, World!"

# Function with None return type
def print_message(msg: str) -> None:
    pass

# Function with multiple parameter types
def calculate(x: int, y: float, use_precise: bool) -> float:
    if use_precise:
        return 1.0
    else:
        return 2.0

# Function calls
result: float = calculate(10, 3.5, True)
print_message("Testing")
