# Test float comparison operators

x: float = 3.14
y: float = 2.71
z: float = 3.14

# Equality
eq_diff: bool = x == y
eq_same: bool = x == z
print(b"3.14 == 2.71:", eq_diff)
print(b"3.14 == 3.14:", eq_same)

# Not equal
ne_diff: bool = x != y
ne_same: bool = x != z
print(b"3.14 != 2.71:", ne_diff)
print(b"3.14 != 3.14:", ne_same)

# Less than
lt1: bool = y < x
lt2: bool = x < y
print(b"2.71 < 3.14:", lt1)
print(b"3.14 < 2.71:", lt2)

# Less than or equal
le1: bool = y <= x
le2: bool = x <= z
le3: bool = x <= y
print(b"2.71 <= 3.14:", le1)
print(b"3.14 <= 3.14:", le2)
print(b"3.14 <= 2.71:", le3)

# Greater than
gt1: bool = x > y
gt2: bool = y > x
print(b"3.14 > 2.71:", gt1)
print(b"2.71 > 3.14:", gt2)

# Greater than or equal
ge1: bool = x >= y
ge2: bool = x >= z
ge3: bool = y >= x
print(b"3.14 >= 2.71:", ge1)
print(b"3.14 >= 3.14:", ge2)
print(b"2.71 >= 3.14:", ge3)

# Comparison with negative floats
a: float = -1.5
b: float = 1.5
neg_cmp: bool = a < b
print(b"-1.5 < 1.5:", neg_cmp)

# Comparison near zero
small1: float = 0.0001
small2: float = 0.0002
small_cmp: bool = small1 < small2
print(b"0.0001 < 0.0002:", small_cmp)

# Chained comparison in condition
def in_range(val: float, low: float, high: float) -> bool:
    return val >= low and val <= high

result: bool = in_range(2.5, 1.0, 5.0)
print(b"2.5 in [1.0, 5.0]:", result)

result2: bool = in_range(0.5, 1.0, 5.0)
print(b"0.5 in [1.0, 5.0]:", result2)
