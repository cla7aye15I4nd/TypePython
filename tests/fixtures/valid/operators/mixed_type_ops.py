# Test mixed type operations to improve coverage

# Float operations with bool
f: float = 5.5
b: bool = True

# Arithmetic with bool
print(f + b)  # float + bool
print(f - b)  # float - bool
print(f * b)  # float * bool
print(f / b)  # float / bool
print(f // b)  # float // bool
print(f % b)  # float % bool
print(f ** b)  # float ** bool

# Comparisons float with bool
if f < b:
    print(0)
elif f <= b:
    print(0)
elif f > b:
    if f >= b:
        print(1)

# Int is/is not operations
a: int = 5
c: int = 5
d: int = 10

if a is c:
    print(2)

if d is not a:
    print(3)

# More logical operations
# Float and bool
x: float = 0.0
y: bool = True
z = x and y  # 0.0 and True -> False (0)

p: float = 2.5
q: bool = False
r = p and q  # 2.5 and False -> False (0)

# Float or bool
s: float = 0.0
t: bool = True
u = s or t  # 0.0 or True -> True (1)

v: float = 3.5
w: bool = False
xx = v or w  # 3.5 or False -> 3.5

print(4)
