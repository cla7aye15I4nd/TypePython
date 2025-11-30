# Test bytes iteration patterns

# Basic bytes iteration (yields integers)
data: bytes = b"hello"
for b in data:
    print(b"Byte value:", b)

# Bytes as integer sequence
byte_vals: bytes = b"\x00\x01\x02\x03\x04"
total: int = 0
for b in byte_vals:
    total = total + b
print(b"Sum of bytes:", total)

# Check byte values
ascii_data: bytes = b"ABC"
for b in ascii_data:
    print(b"ASCII value:", b)

# Iterate with index
binary: bytes = b"\xff\x00\xff\x00"
for i in range(len(binary)):
    print(b"Byte at", i, b":", binary[i])

# Build list from bytes
byte_list: list[int] = []
for b in b"test":
    byte_list.append(b)
print(b"Byte list:", byte_list)

# Count specific byte value
zeros: int = 0
for b in b"\x00\x01\x00\x02\x00\x03":
    if b == 0:
        zeros = zeros + 1
print(b"Zero count:", zeros)

# Bytes in enumerate
for i, b in enumerate(b"abc"):
    print(b"Index:", i, b"Value:", b)

# Bytes in zip
for b1, b2 in zip(b"abc", b"xyz"):
    print(b"Pair:", b1, b2)

# Reversed bytes
for b in reversed(b"hello"):
    print(b"Reversed byte:", b)

# Bytes comprehension to list
hex_vals: list[int] = [b for b in b"\x0a\x0b\x0c"]
print(b"Hex values:", hex_vals)

# Filter bytes
printable: list[int] = [b for b in b"Hello\x00World\x01!" if b >= 32]
print(b"Printable bytes:", printable)
