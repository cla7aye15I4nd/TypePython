# Test None type operations

# None literals and identity
x: None = None
y: None = None

# Two None values are identical
same_none: bool = x is y
print(b"None is None:", same_none)

# Two None values not identical (should be False)
not_same: bool = x is not y
print(b"None is not None:", not_same)

# Equality with None variables
eq_none: bool = x == y
print(b"None == None:", eq_none)

# Not equal with None variables
ne_none: bool = x != y
print(b"None != None:", ne_none)

# None in boolean context (always falsy)
print(b"None in if:")
if x:
    print(b"None is truthy")
else:
    print(b"None is falsy")

# None in collections (always False since collections contain int/str)
print(b"None in list:", None in [1, 2, 3])
print(b"None not in list:", None not in [1, 2, 3])
print(b"None in dict:", None in {1: 10, 2: 20})
print(b"None not in dict:", None not in {1: 10, 2: 20})
print(b"None in set:", None in {1, 2, 3})
print(b"None not in set:", None not in {1, 2, 3})
