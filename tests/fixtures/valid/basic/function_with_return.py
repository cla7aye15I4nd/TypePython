# Function with return value
def add(a: int, b: int) -> int:
    return a + b

def multiply(x: int, y: int) -> int:
    return x * y

sum: int = add(5, 3)
product: int = multiply(4, 7)

print("Sum:", sum)
print("Product:", product)
