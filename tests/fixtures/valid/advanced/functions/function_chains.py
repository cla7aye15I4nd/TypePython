# Chaining function calls
def double(n: int) -> int:
    return n * 2

def triple(n: int) -> int:
    return n * 3

def add_ten(n: int) -> int:
    return n + 10

def square(n: int) -> int:
    return n * n

def apply_operations(n: int) -> int:
    result: int = square(add_ten(triple(double(n))))
    return result

def nested_calls(n: int) -> int:
    a: int = double(n)
    b: int = triple(a)
    c: int = add_ten(b)
    d: int = square(c)
    return d

def complex_chain(x: int) -> int:
    return double(square(triple(x))) + square(double(x))

result1: int = apply_operations(5)
result2: int = nested_calls(5)
result3: int = complex_chain(3)

print("Apply operations on 5:", result1)
print("Nested calls on 5:", result2)
print("Complex chain on 3:", result3)
