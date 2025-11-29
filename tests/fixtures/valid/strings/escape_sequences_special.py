# Test special escape sequences
x: bytes = b"\a\b\f\v"
print(x)
y: bytes = b"\101\102\103"
print(y)
z: bytes = b"\x41\x42\x43"
print(z)
w: bytes = b"\q"
print(w)
