# Test various escape sequences in bytes literals

# Basic escape sequences
print(b"line1\nline2")
print(b"tab\there")
print(b"backslash\\")
print(b"quote\"mark")
print(b"carriage\rreturn")

# ASCII special characters
print(b"bell\achar")
print(b"backspace\bchar")
print(b"formfeed\fchar")
print(b"vertical\vtab")
print(b"apostrophe\'char")

# Hex escape
print(b"\x41\x42\x43")

# Octal escape (single digit)
print(b"\101\102\103")
