# Test boolean identity operators (is, is not)

t: bool = True
f: bool = False

# is operator on bools
same_true: bool = t is True
print(b"True is True:", same_true)

same_false: bool = f is False
print(b"False is False:", same_false)

diff_is: bool = t is False
print(b"True is False:", diff_is)

# is not operator on bools
not_same1: bool = t is not False
print(b"True is not False:", not_same1)

not_same2: bool = f is not True
print(b"False is not True:", not_same2)

same_isnot: bool = t is not True
print(b"True is not True:", same_isnot)

# Comparing variables
a: bool = True
b: bool = True
c: bool = False

vars_same: bool = a is b
print(b"a is b (both True):", vars_same)

vars_diff: bool = a is c
print(b"a is c (True is False):", vars_diff)

vars_not_same: bool = a is not c
print(b"a is not c:", vars_not_same)
