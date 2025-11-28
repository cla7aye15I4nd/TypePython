# Test bitwise operators on booleans

t: bool = True
f: bool = False

# Bitwise OR on booleans
or1: bool = t | t
or2: bool = t | f
or3: bool = f | t
or4: bool = f | f
print(b"True | True:", or1)
print(b"True | False:", or2)
print(b"False | True:", or3)
print(b"False | False:", or4)

# Bitwise AND on booleans
and1: bool = t & t
and2: bool = t & f
and3: bool = f & t
and4: bool = f & f
print(b"True & True:", and1)
print(b"True & False:", and2)
print(b"False & True:", and3)
print(b"False & False:", and4)

# Bitwise XOR on booleans
xor1: bool = t ^ t
xor2: bool = t ^ f
xor3: bool = f ^ t
xor4: bool = f ^ f
print(b"True ^ True:", xor1)
print(b"True ^ False:", xor2)
print(b"False ^ True:", xor3)
print(b"False ^ False:", xor4)

# Combining bitwise and logical
a: bool = True
b: bool = False
c: bool = True

# Bitwise combines all
combined: bool = (a & c) | b
print(b"(True & True) | False:", combined)

# XOR as toggle
toggle: bool = t ^ t
print(b"True ^ True (toggle off):", toggle)

toggle2: bool = f ^ t
print(b"False ^ True (toggle on):", toggle2)
