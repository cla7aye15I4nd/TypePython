# Test float logical operations (and, or)

# Float and float
a: float = 5.5
b: float = 0.0
c = a and b  # Should return 0.0

d: float = 0.0
e: float = 3.3
f = d and e  # Should return 0.0

# Float or float
g: float = 5.5
h: float = 0.0
i = g or h  # Should return 5.5

j: float = 0.0
k: float = 3.3
m = j or k  # Should return 3.3

# Float and int
n: float = 5.5
p: int = 0
q = n and p  # Should return 0

r: float = 0.0
s: int = 5
t = r and s  # Should return 0

# Float or int
u: float = 5.5
v: int = 0
w = u or v  # Should return 5.5

x: float = 0.0
y: int = 5
z = x or y  # Should return 5

print(1)
