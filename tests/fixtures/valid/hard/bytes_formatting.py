# Test bytes printf-style formatting (% operator) - basic cases only

# Format with int using %d
fmt_int: bytes = b"Value: %d"
result_int: bytes = fmt_int % 42
print(len(result_int))

# Format with float using %f
fmt_float: bytes = b"Pi: %f"
result_float: bytes = fmt_float % 3.14159
print(len(result_float))

# Format with bytes using %s
fmt_bytes: bytes = b"Data: %s"
result_bytes: bytes = fmt_bytes % b"binary"
print(len(result_bytes))

# Format with int using %x (hex)
fmt_hex: bytes = b"Hex: %x"
result_hex: bytes = fmt_hex % 255
print(len(result_hex))

# Format with int using %o (octal)
fmt_oct: bytes = b"Oct: %o"
result_oct: bytes = fmt_oct % 64
print(len(result_oct))

# Format with negative number
fmt_neg: bytes = b"Neg: %d"
result_neg: bytes = fmt_neg % -42
print(len(result_neg))
