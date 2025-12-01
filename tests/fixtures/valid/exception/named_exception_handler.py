# Test named exception handler
try:
    raise ValueError
except ValueError as e:
    print(b"caught ValueError")
print(b"done")
