# Test function manipulation - passing functions as values
# Expected output:
# 15
# 8
# HELLO
# b'world     '

# Test 1: Store a user-defined function in a variable
def add(a: int, b: int) -> int:
    return a + b

f = add
result: int = f(10, 5)
print(result)

# Test 2: Store and call another function
def multiply(x: int, y: int) -> int:
    return x * y

g = multiply
result2: int = g(2, 4)
print(result2)

# Test 3: Bound method stored in variable (str)
s: str = "hello"
upper_method = s.upper
result3: str = upper_method()
print(result3)

# Test 4: Bound method stored in variable (bytes)
b: bytes = b"world"
ljust_method = b.ljust
result4: bytes = ljust_method(10)
print(result4)
