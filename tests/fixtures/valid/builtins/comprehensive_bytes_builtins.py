# Comprehensive test for all bytes operations with all applicable builtin functions
# Tests: len, min/max on byte values
# Bytes methods: count, startswith, endswith, upper, lower, capitalize, title,
#                swapcase, strip, lstrip, rstrip, replace, center, ljust, rjust,
#                zfill, isalpha, isdigit, isalnum, isspace, isupper, islower

# ============================================================================
# SECTION 1: len() with bytes
# ============================================================================
print("# len() with bytes")

empty_bytes: bytes = b""
print(len(empty_bytes))  # 0

single_byte: bytes = b"A"
print(len(single_byte))  # 1

multi_bytes: bytes = b"Hello"
print(len(multi_bytes))  # 5

longer_bytes: bytes = b"Hello, World!"
print(len(longer_bytes))  # 13

# ============================================================================
# SECTION 2: Bytes indexing
# ============================================================================
print("# Bytes indexing")

idx_bytes: bytes = b"Python"
print(idx_bytes[0])  # 80 (ord('P'))
print(idx_bytes[1])  # 121 (ord('y'))
print(idx_bytes[5])  # 110 (ord('n'))

# ============================================================================
# SECTION 3: min() and max() with byte values
# ============================================================================
print("# min() and max() with byte values")

byte_vals: bytes = b"abc"
# Get individual byte values and compare
b0: int = byte_vals[0]  # 97 ('a')
b1: int = byte_vals[1]  # 98 ('b')
b2: int = byte_vals[2]  # 99 ('c')

print(min(b0, b1))  # 97
print(max(b1, b2))  # 99
print(min(b0, b1, b2))  # 97
print(max(b0, b1, b2))  # 99

# ============================================================================
# SECTION 4: Bytes method - count()
# ============================================================================
print("# count() method")

count_bytes: bytes = b"hello world hello"
count1: int = count_bytes.count(b"hello")
print(count1)  # 2

count2: int = count_bytes.count(b"world")
print(count2)  # 1

count3: int = count_bytes.count(b"x")
print(count3)  # 0

# Single byte count
count4: int = count_bytes.count(b"l")
print(count4)  # 5

# ============================================================================
# SECTION 5: Bytes method - upper()
# ============================================================================
print("# upper() method")

lower_bytes: bytes = b"hello"
upper_bytes: bytes = lower_bytes.upper()
print(len(upper_bytes))  # 5
print(upper_bytes[0])  # 72 ('H')

# ============================================================================
# SECTION 6: Bytes method - lower()
# ============================================================================
print("# lower() method")

mixed_bytes: bytes = b"HeLLo"
lowered: bytes = mixed_bytes.lower()
print(len(lowered))  # 5
print(lowered[0])  # 104 ('h')

# ============================================================================
# SECTION 7: Bytes method - capitalize()
# ============================================================================
print("# capitalize() method")

cap_bytes: bytes = b"hello world"
capitalized: bytes = cap_bytes.capitalize()
print(len(capitalized))  # 11
print(capitalized[0])  # 72 ('H')

# ============================================================================
# SECTION 8: Bytes method - title()
# ============================================================================
print("# title() method")

title_bytes: bytes = b"hello world"
titled: bytes = title_bytes.title()
print(len(titled))  # 11
print(titled[0])  # 72 ('H')
print(titled[6])  # 87 ('W')

# ============================================================================
# SECTION 9: Bytes method - swapcase()
# ============================================================================
print("# swapcase() method")

swap_bytes: bytes = b"HeLLo"
swapped: bytes = swap_bytes.swapcase()
print(len(swapped))  # 5
print(swapped[0])  # 104 ('h')
print(swapped[2])  # 76 ('L')

# ============================================================================
# SECTION 10: Bytes method - strip()
# ============================================================================
print("# strip() method")

strip_bytes: bytes = b"  hello  "
stripped: bytes = strip_bytes.strip()
print(len(stripped))  # 5

strip_bytes2: bytes = b"xxxhelloxxx"
stripped2: bytes = strip_bytes2.strip(b"x")
print(len(stripped2))  # 5

# ============================================================================
# SECTION 11: Bytes method - lstrip()
# ============================================================================
print("# lstrip() method")

lstrip_bytes: bytes = b"  hello  "
lstripped: bytes = lstrip_bytes.lstrip()
print(len(lstripped))  # 7 (removed leading spaces)

lstrip_bytes2: bytes = b"xxxhello"
lstripped2: bytes = lstrip_bytes2.lstrip(b"x")
print(len(lstripped2))  # 5

# ============================================================================
# SECTION 12: Bytes method - rstrip()
# ============================================================================
print("# rstrip() method")

rstrip_bytes: bytes = b"  hello  "
rstripped: bytes = rstrip_bytes.rstrip()
print(len(rstripped))  # 7 (removed trailing spaces)

rstrip_bytes2: bytes = b"helloxxx"
rstripped2: bytes = rstrip_bytes2.rstrip(b"x")
print(len(rstripped2))  # 5

# ============================================================================
# SECTION 13: Bytes method - replace()
# ============================================================================
print("# replace() method")

replace_bytes: bytes = b"hello world"
replaced: bytes = replace_bytes.replace(b"world", b"there")
print(len(replaced))  # 11

replace_bytes2: bytes = b"aaa"
replaced2: bytes = replace_bytes2.replace(b"a", b"b")
print(len(replaced2))  # 3
print(replaced2[0])  # 98 ('b')

# ============================================================================
# SECTION 14: Bytes method - startswith()
# ============================================================================
print("# startswith() method")

start_bytes: bytes = b"hello world"
starts1: bool = start_bytes.startswith(b"hello")
print(starts1)  # True

starts2: bool = start_bytes.startswith(b"world")
print(starts2)  # False

starts3: bool = start_bytes.startswith(b"h")
print(starts3)  # True

# ============================================================================
# SECTION 15: Bytes method - endswith()
# ============================================================================
print("# endswith() method")

end_bytes: bytes = b"hello world"
ends1: bool = end_bytes.endswith(b"world")
print(ends1)  # True

ends2: bool = end_bytes.endswith(b"hello")
print(ends2)  # False

ends3: bool = end_bytes.endswith(b"d")
print(ends3)  # True

# ============================================================================
# SECTION 16: Bytes method - center()
# ============================================================================
print("# center() method")

center_bytes: bytes = b"hi"
centered: bytes = center_bytes.center(10)
print(len(centered))  # 10

centered2: bytes = center_bytes.center(5, b"*")
print(len(centered2))  # 5

# ============================================================================
# SECTION 17: Bytes method - ljust()
# ============================================================================
print("# ljust() method")

ljust_bytes: bytes = b"hi"
ljusted: bytes = ljust_bytes.ljust(10)
print(len(ljusted))  # 10

ljusted2: bytes = ljust_bytes.ljust(5, b"*")
print(len(ljusted2))  # 5

# ============================================================================
# SECTION 18: Bytes method - rjust()
# ============================================================================
print("# rjust() method")

rjust_bytes: bytes = b"hi"
rjusted: bytes = rjust_bytes.rjust(10)
print(len(rjusted))  # 10

rjusted2: bytes = rjust_bytes.rjust(5, b"*")
print(len(rjusted2))  # 5

# ============================================================================
# SECTION 19: Bytes method - zfill()
# ============================================================================
print("# zfill() method")

zfill_bytes: bytes = b"42"
zfilled: bytes = zfill_bytes.zfill(5)
print(len(zfilled))  # 5

zfill_bytes2: bytes = b"-42"
zfilled2: bytes = zfill_bytes2.zfill(5)
print(len(zfilled2))  # 5

# ============================================================================
# SECTION 20: Bytes method - isalpha()
# ============================================================================
print("# isalpha() method")

alpha_bytes: bytes = b"hello"
is_alpha1: bool = alpha_bytes.isalpha()
print(is_alpha1)  # True

alpha_bytes2: bytes = b"hello123"
is_alpha2: bool = alpha_bytes2.isalpha()
print(is_alpha2)  # False

# ============================================================================
# SECTION 21: Bytes method - isdigit()
# ============================================================================
print("# isdigit() method")

digit_bytes: bytes = b"12345"
is_digit1: bool = digit_bytes.isdigit()
print(is_digit1)  # True

digit_bytes2: bytes = b"123abc"
is_digit2: bool = digit_bytes2.isdigit()
print(is_digit2)  # False

# ============================================================================
# SECTION 22: Bytes method - isalnum()
# ============================================================================
print("# isalnum() method")

alnum_bytes: bytes = b"hello123"
is_alnum1: bool = alnum_bytes.isalnum()
print(is_alnum1)  # True

alnum_bytes2: bytes = b"hello 123"
is_alnum2: bool = alnum_bytes2.isalnum()
print(is_alnum2)  # False (space)

# ============================================================================
# SECTION 23: Bytes method - isspace()
# ============================================================================
print("# isspace() method")

space_bytes: bytes = b"   "
is_space1: bool = space_bytes.isspace()
print(is_space1)  # True

space_bytes2: bytes = b" a "
is_space2: bool = space_bytes2.isspace()
print(is_space2)  # False

# ============================================================================
# SECTION 24: Bytes method - isupper()
# ============================================================================
print("# isupper() method")

upper_test: bytes = b"HELLO"
is_upper1: bool = upper_test.isupper()
print(is_upper1)  # True

upper_test2: bytes = b"Hello"
is_upper2: bool = upper_test2.isupper()
print(is_upper2)  # False

# ============================================================================
# SECTION 25: Bytes method - islower()
# ============================================================================
print("# islower() method")

lower_test: bytes = b"hello"
is_lower1: bool = lower_test.islower()
print(is_lower1)  # True

lower_test2: bytes = b"Hello"
is_lower2: bool = lower_test2.islower()
print(is_lower2)  # False

# ============================================================================
# SECTION 26: Complex combinations
# ============================================================================
print("# Complex combinations")

combo_bytes: bytes = b"  Hello World  "
# Strip and get length
stripped_combo: bytes = combo_bytes.strip()
print(len(stripped_combo))  # 11

# Convert to upper and check length
upper_combo: bytes = stripped_combo.upper()
print(len(upper_combo))  # 11

# Count specific byte
count_o: int = upper_combo.count(b"O")
print(count_o)  # 2

# Replace and check
replaced_combo: bytes = upper_combo.replace(b"WORLD", b"THERE")
print(len(replaced_combo))  # 11

# ============================================================================
# SECTION 27: Chained operations
# ============================================================================
print("# Chained operations")

chain_bytes: bytes = b"  hello  "
step1: bytes = chain_bytes.strip()
print(len(step1))  # 5

step2: bytes = step1.upper()
print(len(step2))  # 5

step3: bytes = step2.center(10)
print(len(step3))  # 10

# ============================================================================
# SECTION 28: Using abs() with byte values
# ============================================================================
print("# abs() with byte values")

# Get byte values and use abs (though they're already positive)
abs_bytes: bytes = b"ABC"
val_a: int = abs_bytes[0]
print(abs(val_a))  # 65 (same as val_a)

# Using with arithmetic
diff: int = abs_bytes[0] - abs_bytes[1]
print(abs(diff))  # abs(65-66) = 1

# ============================================================================
# SECTION 29: Using min/max with multiple byte values
# ============================================================================
print("# min/max with multiple byte values")

minmax_bytes: bytes = b"ZYXABC"
v0: int = minmax_bytes[0]  # 90 ('Z')
v1: int = minmax_bytes[1]  # 89 ('Y')
v2: int = minmax_bytes[2]  # 88 ('X')
v3: int = minmax_bytes[3]  # 65 ('A')

print(min(v0, v1, v2, v3))  # 65
print(max(v0, v1, v2, v3))  # 90

# ============================================================================
# SECTION 30: Predicates with empty and non-empty bytes
# ============================================================================
print("# Predicates with various bytes")

# Empty bytes
empty: bytes = b""
print(len(empty))  # 0

# Numeric bytes
numeric: bytes = b"999"
print(numeric.isdigit())  # True
print(len(numeric))  # 3

# Alphabetic bytes
alpha_only: bytes = b"xyz"
print(alpha_only.isalpha())  # True
print(len(alpha_only))  # 3

# ============================================================================
# SECTION 31: Edge cases with padding methods
# ============================================================================
print("# Edge cases with padding")

pad_bytes: bytes = b"x"
# Center with even padding
centered_even: bytes = pad_bytes.center(6, b"-")
print(len(centered_even))  # 6

# Ljust
ljust_pad: bytes = pad_bytes.ljust(8, b".")
print(len(ljust_pad))  # 8

# Rjust
rjust_pad: bytes = pad_bytes.rjust(8, b".")
print(len(rjust_pad))  # 8

# ============================================================================
# SECTION 32: Multiple replacements
# ============================================================================
print("# Multiple replacements")

multi_replace: bytes = b"aaa bbb aaa"
rep1: bytes = multi_replace.replace(b"aaa", b"xxx")
print(len(rep1))  # 11 (same length)
print(rep1.count(b"xxx"))  # 2

rep2: bytes = rep1.replace(b"bbb", b"yyy")
print(len(rep2))  # 11

# ============================================================================
# SECTION 33: Combining predicates and transformations
# ============================================================================
print("# Combining predicates and transformations")

pred_bytes: bytes = b"HELLO"
print(pred_bytes.isupper())  # True

lowered_pred: bytes = pred_bytes.lower()
print(lowered_pred.isupper())  # False
print(lowered_pred.islower())  # True

# ============================================================================
# SECTION 34: pow() with byte values
# ============================================================================
print("# pow() with byte values")

pow_bytes: bytes = b"\x02\x03"  # bytes with values 2 and 3
byte_val1: int = pow_bytes[0]  # 2
byte_val2: int = pow_bytes[1]  # 3
print(pow(byte_val1, byte_val2))  # 8

# ============================================================================
# SECTION 35: round() conceptually (not directly applicable to bytes)
# But we can demonstrate with float operations
# ============================================================================
print("# round() with floats (for completeness)")

print(round(3.14159, 2))  # 3.14

print("# All tests completed")
