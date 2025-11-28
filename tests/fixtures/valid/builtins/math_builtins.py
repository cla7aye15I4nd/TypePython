# Test Python built-in math functions

# Test abs()
print(abs(5))      # 5
print(abs(-5))     # 5
print(abs(0))      # 0
print(abs(-3.14))  # 3.14
print(abs(3.14))   # 3.14

# Test round()
print(round(3.7))      # 4
print(round(3.2))      # 3
print(round(-3.7))     # -4
print(round(5))        # 5 (int unchanged)
print(round(3.14159, 2))  # 3.14

# Test min()
print(min(5, 3))       # 3
print(min(-5, -3))     # -5
print(min(3.14, 2.71)) # 2.71
print(min(5, 3.0))     # 3.0 (coerced to float)

# Test max()
print(max(5, 3))       # 5
print(max(-5, -3))     # -3
print(max(3.14, 2.71)) # 3.14
print(max(5, 3.0))     # 5.0 (coerced to float)

# Test pow()
print(pow(2, 3))       # 8
print(pow(2, 10))      # 1024
print(pow(2.0, 3.0))   # 8.0
print(pow(2, 3, 5))    # 3 (modular exponentiation: 8 % 5 = 3)
print(pow(3, 7, 11))   # 9 (3^7 = 2187, 2187 % 11 = 9)
