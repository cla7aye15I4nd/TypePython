# Test truthiness/faliness conversion of different types

# Int truthiness
i: int = 5
if i:
    print(b"int 5 is truthy")
i = 0
if i:
    print(b"int 0 is truthy")
else:
    print(b"int 0 is falsy")

# Float truthiness
f: float = 3.14
if f:
    print(b"float 3.14 is truthy")
f = 0.0
if f:
    print(b"float 0.0 is truthy")
else:
    print(b"float 0.0 is falsy")

# Str truthiness
s: str = "hello"
if s:
    print(b"str 'hello' is truthy")
s = ""
if s:
    print(b"str '' is truthy")
else:
    print(b"str '' is falsy")

# Bytes truthiness
bs: bytes = b"hello"
if bs:
    print(b"bytes b'hello' is truthy")
bs = b""
if bs:
    print(b"bytes b'' is truthy")
else:
    print(b"bytes b'' is falsy")

# List truthiness
l: list[int] = [1, 2, 3]
if l:
    print(b"list [1,2,3] is truthy")
l = []
if l:
    print(b"list [] is truthy")
else:
    print(b"list [] is falsy")

# Dict truthiness
d: dict[str, int] = {"a": 1}
if d:
    print(b"dict {'a':1} is truthy")
d = {}
if d:
    print(b"dict {} is truthy")
else:
    print(b"dict {} is falsy")

# Set truthiness
st: set[int] = {1, 2, 3}
if st:
    print(b"set {1,2,3} is truthy")
st = set()
if st:
    print(b"set {} is truthy")
else:
    print(b"set {} is falsy")
