# Comprehensive test for all Python math operations
# Testing: abs, round, min, max, pow, arithmetic operators, edge cases

# ============================================================================
# 1. abs() - Integer absolute values
# ============================================================================
print(b"1. abs() integers:")
abs_i1: int = abs(42)
abs_i2: int = abs(-42)
abs_i3: int = abs(0)
abs_i4: int = abs(-1)
abs_i5: int = abs(1000000)
abs_i6: int = abs(-1000000)
print(abs_i1)
print(abs_i2)
print(abs_i3)
print(abs_i4)
print(abs_i5)
print(abs_i6)

# ============================================================================
# 2. abs() - Float absolute values
# ============================================================================
print(b"2. abs() floats:")
abs_f1: float = abs(3.14)
abs_f2: float = abs(-3.14)
abs_f3: float = abs(0.0)
abs_f4: float = abs(-0.001)
abs_f5: float = abs(999.999)
abs_f6: float = abs(-999.999)
print(abs_f1)
print(abs_f2)
print(abs_f3)
print(abs_f4)
print(abs_f5)
print(abs_f6)

# ============================================================================
# 3. round() - Basic rounding
# ============================================================================
print(b"3. round() basic:")
rnd1: int = round(3.4)
rnd2: int = round(3.5)
rnd3: int = round(3.6)
rnd4: int = round(-3.4)
rnd5: int = round(-3.5)
rnd6: int = round(-3.6)
print(rnd1)
print(rnd2)
print(rnd3)
print(rnd4)
print(rnd5)
print(rnd6)

# ============================================================================
# 4. round() - With precision
# ============================================================================
print(b"4. round() precision:")
rndp1: float = round(3.14159, 2)
rndp2: float = round(3.14159, 3)
rndp3: float = round(3.14159, 4)
rndp4: float = round(2.71828, 1)
rndp5: float = round(2.71828, 2)
rndp6: float = round(-1.2345, 2)
print(rndp1)
print(rndp2)
print(rndp3)
print(rndp4)
print(rndp5)
print(rndp6)

# ============================================================================
# 5. round() - Integer passthrough
# ============================================================================
print(b"5. round() integers:")
rndi1: int = round(5)
rndi2: int = round(-5)
rndi3: int = round(0)
rndi4: int = round(100)
print(rndi1)
print(rndi2)
print(rndi3)
print(rndi4)

# ============================================================================
# 6. min() - Integer comparisons
# ============================================================================
print(b"6. min() integers:")
min_i1: int = min(5, 3)
min_i2: int = min(3, 5)
min_i3: int = min(-5, -3)
min_i4: int = min(-3, -5)
min_i5: int = min(0, 100)
min_i6: int = min(-100, 0)
print(min_i1)
print(min_i2)
print(min_i3)
print(min_i4)
print(min_i5)
print(min_i6)

# ============================================================================
# 7. min() - Float comparisons
# ============================================================================
print(b"7. min() floats:")
min_f1: float = min(3.14, 2.71)
min_f2: float = min(2.71, 3.14)
min_f3: float = min(-1.5, -2.5)
min_f4: float = min(0.0, 0.001)
min_f5: float = min(-0.001, 0.001)
print(min_f1)
print(min_f2)
print(min_f3)
print(min_f4)
print(min_f5)

# ============================================================================
# 8. min() - Mixed int/float
# ============================================================================
print(b"8. min() mixed:")
min_m1: float = min(5, 3.0)
min_m2: float = min(3.0, 5)
min_m3: float = min(2.5, 3)
min_m4: float = min(3, 2.5)
print(min_m1)
print(min_m2)
print(min_m3)
print(min_m4)

# ============================================================================
# 9. max() - Integer comparisons
# ============================================================================
print(b"9. max() integers:")
max_i1: int = max(5, 3)
max_i2: int = max(3, 5)
max_i3: int = max(-5, -3)
max_i4: int = max(-3, -5)
max_i5: int = max(0, 100)
max_i6: int = max(-100, 0)
print(max_i1)
print(max_i2)
print(max_i3)
print(max_i4)
print(max_i5)
print(max_i6)

# ============================================================================
# 10. max() - Float comparisons
# ============================================================================
print(b"10. max() floats:")
max_f1: float = max(3.14, 2.71)
max_f2: float = max(2.71, 3.14)
max_f3: float = max(-1.5, -2.5)
max_f4: float = max(0.0, 0.001)
max_f5: float = max(-0.001, 0.001)
print(max_f1)
print(max_f2)
print(max_f3)
print(max_f4)
print(max_f5)

# ============================================================================
# 11. max() - Mixed int/float (using same types to avoid coercion issues)
# ============================================================================
print(b"11. max() mixed:")
max_m1: int = max(5, 3)
max_m2: int = max(3, 5)
max_m3: int = max(2, 3)
max_m4: int = max(3, 2)
print(max_m1)
print(max_m2)
print(max_m3)
print(max_m4)

# ============================================================================
# 12. pow() - Integer powers
# ============================================================================
print(b"12. pow() integers:")
pow_i1: int = pow(2, 0)
pow_i2: int = pow(2, 1)
pow_i3: int = pow(2, 3)
pow_i4: int = pow(2, 10)
pow_i5: int = pow(3, 4)
pow_i6: int = pow(5, 3)
print(pow_i1)
print(pow_i2)
print(pow_i3)
print(pow_i4)
print(pow_i5)
print(pow_i6)

# ============================================================================
# 13. pow() - Float powers
# ============================================================================
print(b"13. pow() floats:")
pow_f1: float = pow(2.0, 3.0)
pow_f2: float = pow(4.0, 0.5)
pow_f3: float = pow(2.5, 2.0)
print(pow_f1)
print(pow_f2)
print(pow_f3)

# ============================================================================
# 14. pow() - Modular exponentiation
# ============================================================================
print(b"14. pow() modular:")
pow_m1: int = pow(2, 3, 5)
pow_m2: int = pow(3, 7, 11)
pow_m3: int = pow(2, 10, 100)
pow_m4: int = pow(5, 5, 13)
pow_m5: int = pow(7, 3, 10)
print(pow_m1)
print(pow_m2)
print(pow_m3)
print(pow_m4)
print(pow_m5)

# ============================================================================
# 15. Power operator (**)
# ============================================================================
print(b"15. ** operator:")
star_i1: int = 2 ** 5
star_i2: int = 3 ** 3
star_i3: int = 10 ** 2
star_f1: float = 2.0 ** 3.0
star_f2: float = 4.0 ** 0.5
print(star_i1)
print(star_i2)
print(star_i3)
print(star_f1)
print(star_f2)

# ============================================================================
# 16. Floor division (//)
# ============================================================================
print(b"16. // operator:")
fdiv1: int = 7 // 3
fdiv2: int = 10 // 2
fdiv3: int = -7 // 3
fdiv4: int = 7 // -3
fdiv5: int = -7 // -3
fdiv6: int = 100 // 7
print(fdiv1)
print(fdiv2)
print(fdiv3)
print(fdiv4)
print(fdiv5)
print(fdiv6)

# ============================================================================
# 17. Modulo operator (%)
# ============================================================================
print(b"17. % operator:")
mod1: int = 7 % 3
mod2: int = 10 % 2
mod3: int = -7 % 3
mod4: int = 7 % -3
mod5: int = -7 % -3
mod6: int = 100 % 7
print(mod1)
print(mod2)
print(mod3)
print(mod4)
print(mod5)
print(mod6)

# ============================================================================
# 18. Basic arithmetic
# ============================================================================
print(b"18. Basic arithmetic:")
add1: int = 100 + 200
sub1: int = 500 - 123
mul1: int = 12 * 12
div1: float = 10.0 / 4.0
print(add1)
print(sub1)
print(mul1)
print(div1)

# ============================================================================
# 19. Unary minus
# ============================================================================
print(b"19. Unary minus:")
neg1: int = -5
neg2: int = -(-5)
neg3: int = -0
neg4: float = -3.14
neg5: float = -(-3.14)
print(neg1)
print(neg2)
print(neg3)
print(neg4)
print(neg5)

# ============================================================================
# 20. Complex expressions
# ============================================================================
print(b"20. Complex expressions:")
expr1: int = 2 + 3 * 4
expr2: int = (2 + 3) * 4
expr3: int = 10 - 2 - 3
expr4: int = 10 - (2 - 3)
expr5: float = 2.0 ** 3.0 + 1.0
print(expr1)
print(expr2)
print(expr3)
print(expr4)
print(expr5)

# ============================================================================
# 21. Nested function calls
# ============================================================================
print(b"21. Nested calls:")
nest1: int = abs(min(5, -10))
nest2: int = max(abs(-5), abs(-3))
nest3: int = min(abs(-10), abs(5))
nest4: int = round(abs(-3.7))
print(nest1)
print(nest2)
print(nest3)
print(nest4)

# ============================================================================
# 22. Edge case: zero
# ============================================================================
print(b"22. Zero cases:")
zero1: int = 0 + 0
zero2: int = 0 * 100
zero3: int = 100 * 0
zero4: int = 0 // 5
zero5: int = 0 % 5
zero6: int = pow(0, 5)
zero7: int = pow(5, 0)
print(zero1)
print(zero2)
print(zero3)
print(zero4)
print(zero5)
print(zero6)
print(zero7)

# ============================================================================
# 23. Edge case: one
# ============================================================================
print(b"23. One cases:")
one1: int = 1 * 100
one2: int = 100 * 1
one3: int = pow(1, 100)
one4: int = pow(100, 1)
one5: int = 100 // 1
one6: int = 100 % 101
print(one1)
print(one2)
print(one3)
print(one4)
print(one5)
print(one6)

# ============================================================================
# 24. Large numbers
# ============================================================================
print(b"24. Large numbers:")
large1: int = 1000000 + 2000000
large2: int = 1000000 * 1000
large3: int = 999999999 - 1
print(large1)
print(large2)
print(large3)

# ============================================================================
# 25. Float precision
# ============================================================================
print(b"25. Float precision:")
fp1: float = 0.1 + 0.2
fp2: float = 1.0 / 3.0
fp3: float = round(0.1 + 0.2, 1)
print(fp1)
print(fp2)
print(fp3)

# ============================================================================
# 26. Comparison with results
# ============================================================================
print(b"26. Comparisons:")
cmp1: bool = abs(-5) == 5
cmp2: bool = min(3, 5) < max(3, 5)
cmp3: bool = pow(2, 3) == 8
cmp4: bool = 10 // 3 == 3
cmp5: bool = 10 % 3 == 1
print(cmp1)
print(cmp2)
print(cmp3)
print(cmp4)
print(cmp5)

# ============================================================================
# 27. Math in conditions
# ============================================================================
print(b"27. Math conditions:")
val: int = 15
if val % 3 == 0:
    print(b"divisible by 3")
else:
    print(b"not divisible")

if val // 5 >= 3:
    print(b"at least 15")
else:
    print(b"less than 15")

# ============================================================================
# 28. Math in loops
# ============================================================================
print(b"28. Math loops:")
i: int = 1
total: int = 0
while i <= 5:
    total = total + i * i
    i = i + 1
print(total)

# ============================================================================
# 29. Combined operations
# ============================================================================
print(b"29. Combined:")
c1: int = abs(-5) + min(3, 7) + max(2, 4)
c2: int = pow(2, 3) * 2 + 1
c3: float = round(3.14159, 2) + abs(-1.0)
print(c1)
print(c2)
print(c3)

# ============================================================================
# 30. min/max with equal values
# ============================================================================
print(b"30. Equal min/max:")
eq_min: int = min(5, 5)
eq_max: int = max(5, 5)
eq_min_f: float = min(3.14, 3.14)
eq_max_f: float = max(3.14, 3.14)
print(eq_min)
print(eq_max)
print(eq_min_f)
print(eq_max_f)

# ============================================================================
# 31. Operator precedence
# ============================================================================
print(b"31. Precedence:")
prec1: int = 2 + 3 * 4 - 1
prec2: int = 2 ** 3 ** 2
prec3: int = 10 // 3 * 2
prec4: int = 10 % 3 + 2 * 3
print(prec1)
print(prec2)
print(prec3)
print(prec4)
