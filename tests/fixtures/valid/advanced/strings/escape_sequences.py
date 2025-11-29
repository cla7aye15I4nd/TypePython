# Test various escape sequences in strings

# Simple escape sequences
s1: bytes = b"Hello\nWorld"
print(s1)

s2: bytes = b"Tab\there"
print(s2)

s3: bytes = b"Carriage\rReturn"
print(s3)

s4: bytes = b"Back\\slash"
print(s4)

s5: bytes = b"Double\"Quote"
print(s5)

# Hex escape sequences
s7: bytes = b"Hex\x41BC"
print(s7)

s8: bytes = b"Hex\x20Space"
print(s8)

# Octal escape sequences
s9: bytes = b"Octal\101BC"
print(s9)

s10: bytes = b"Octal\040Space"
print(s10)

# Mixed escape sequences with hex and octal
s11: bytes = b"Mix\n\t\x41\101"
print(s11)
