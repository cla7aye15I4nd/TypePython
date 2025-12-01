# Test raise with message
try:
    raise ValueError("custom message")
except ValueError:
    print(b"caught ValueError with message")
print(b"done")
