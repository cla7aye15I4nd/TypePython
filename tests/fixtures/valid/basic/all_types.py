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

# Bytes type (C-style null-terminated strings)
name: bytes = b"TypePython"
message: bytes = b"Hello, World!"

# Function with None return type
def print_message(msg: bytes) -> None:
    print(msg)

# Function with multiple parameter types
def calculate(x: int, y: float, use_precise: bool) -> float:
    if use_precise:
        return 1.0
    else:
        return 2.0

# Function calls
result: float = calculate(10, 3.5, True)
print_message(b"Testing")

# Print all the values
print(b"Age:", age)
print(b"Count:", count)
print(b"Temperature:", temperature)
print(b"Price:", price)
print(b"Is valid:", is_valid)
print(b"Is complete:", is_complete)
print(b"Name:", name)
print(b"Message:", message)
print(b"Result:", result)
print(True < 1)
print(False > 0)
print(True <= 1)
print(False >= 0)
print(True == 1)
print(False != 0)