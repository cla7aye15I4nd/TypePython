# Test additional math builtin cases

# round(int, ndigits) - converts int to float for rounding
print(round(123, 1))    # 123.0
print(round(123, -1))   # 120.0
print(round(125, -1))   # 130.0 (banker's rounding)

# max with mixed int/float types
print(max(1, 2.5))      # 2.5
print(max(3.5, 2))      # 3.5
print(max(1.0, 2.0))    # 2.0 (float, float)
