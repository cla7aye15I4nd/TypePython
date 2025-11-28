# Test variable shadowing - same name in different functions
def func_a() -> int:
    x: int = 10
    print("func_a x:", x)
    return x

def func_b() -> int:
    x: int = 20
    print("func_b x:", x)
    return x

def func_c() -> int:
    x: int = 30
    y: int = x * 2
    print("func_c x:", x)
    print("func_c y:", y)
    return y

# Test parameter shadowing across functions
def process_a(value: int) -> int:
    print("process_a value:", value)
    return value + 100

def process_b(value: int) -> int:
    print("process_b value:", value)
    return value + 200

# Test local variables with same names
def calculate_a() -> int:
    temp: int = 5
    result: int = temp * 2
    print("calculate_a temp:", temp)
    print("calculate_a result:", result)
    return result

def calculate_b() -> int:
    temp: int = 10
    result: int = temp * 3
    print("calculate_b temp:", temp)
    print("calculate_b result:", result)
    return result

# Main execution
print("=== Test 1: Variable Shadowing Across Functions ===")
a: int = func_a()
b: int = func_b()
c: int = func_c()
print("Results:", a, b, c)
print()

print("=== Test 2: Parameter Shadowing ===")
p1: int = process_a(5)
p2: int = process_b(5)
print("Process results:", p1, p2)
print()

print("=== Test 3: Local Variable Shadowing ===")
calc_a: int = calculate_a()
calc_b: int = calculate_b()
print("Calculate results:", calc_a, calc_b)
print()

# Test variable reassignment in same scope
print("=== Test 4: Variable Reassignment ===")
z: int = 1
print("z initially:", z)
z = 2
print("z after first reassign:", z)
z = 3
print("z after second reassign:", z)
