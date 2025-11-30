# Test comprehensive bytes iteration patterns

# Empty bytes
empty: bytes = b""
for b in empty:
    print(b"Never")
print(b"Empty bytes done")

# Single byte
single: bytes = b"x"
for b in single:
    print(b"Single byte:", b)

# ASCII iteration
ascii_bytes: bytes = b"Hello"
for b in ascii_bytes:
    print(b"ASCII:", b, chr(b))

# Binary data
binary: bytes = b"\x00\x01\x02\x03\x04\x05"
for b in binary:
    print(b"Binary:", b)

# Sum of bytes
data: bytes = b"\x01\x02\x03\x04\x05"
total: int = 0
for b in data:
    total = total + b
print(b"Sum:", total)

# Find byte positions
target: bytes = b"banana"
a_byte: int = ord("a")
positions: list[int] = []
for i, b in enumerate(target):
    if b == a_byte:
        positions.append(i)
print(b"'a' positions:", positions)

# Byte frequency
freq: dict[int, int] = {}
for b in b"mississippi":
    if b in freq:
        freq[b] = freq[b] + 1
    else:
        freq[b] = 1
print(b"Byte frequency:", freq)

# Filter printable bytes
raw: bytes = b"Hello\x00World\x01!"
printable: list[int] = []
for b in raw:
    if b >= 32 and b < 127:
        printable.append(b)
print(b"Printable:", printable)

# Build list of bytes
byte_list: list[int] = []
for b in b"test":
    byte_list.append(b)
print(b"Byte list:", byte_list)

# XOR operation
key: int = 0x42
data2: bytes = b"secret"
encrypted: list[int] = []
for b in data2:
    encrypted.append(b ^ key)
print(b"XOR encrypted:", encrypted)

# Decode manually
encoded: bytes = b"\x48\x65\x6c\x6c\x6f"
decoded: str = ""
for b in encoded:
    decoded = decoded + chr(b)
print(b"Decoded:", decoded)

# Count specific byte
zeros: int = 0
for b in b"\x00\x01\x00\x02\x00\x03\x00":
    if b == 0:
        zeros = zeros + 1
print(b"Zero count:", zeros)

# Bytes with enumerate
for i, b in enumerate(b"abc"):
    print(b"Index:", i, b"Byte:", b)

# Bytes with zip
for b1, b2 in zip(b"abc", b"xyz"):
    print(b"Pair:", b1, b2)

# Reversed bytes
for b in reversed(b"hello"):
    print(b"Reversed:", b)

# Check all ASCII
all_ascii: bool = True
test: bytes = b"Hello World"
for b in test:
    if b >= 128:
        all_ascii = False
        break
print(b"All ASCII:", all_ascii)

# Hex dump pattern
hex_data: bytes = b"\xde\xad\xbe\xef"
for i, b in enumerate(hex_data):
    print(b"Offset", i, b":", b)

# Byte comparison
b1: bytes = b"hello"
b2: bytes = b"hallo"
diff_pos: list[int] = []
for i in range(min(len(b1), len(b2))):
    if b1[i] != b2[i]:
        diff_pos.append(i)
print(b"Diff positions:", diff_pos)
