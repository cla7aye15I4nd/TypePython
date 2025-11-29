# Test bytes method calls

# Test upper/lower on variable
s: bytes = b"hello"
print(s.upper())  # HELLO
print(s.lower())  # hello

# Test upper/lower on literal
print(b"World".upper())  # WORLD
print(b"WORLD".lower())  # world

# Mixed case
print(b"HeLLo WoRLd".upper())  # HELLO WORLD
print(b"HeLLo WoRLd".lower())  # hello world

# Test ljust
print(b"hi".ljust(5))   # "hi   "
print(b"hello".ljust(3))  # "hello" (no padding, already >= width)
print(b"x".ljust(4))    # "x   "

# Test rjust
print(b"hi".rjust(5))   # "   hi"
print(b"hello".rjust(3))  # "hello" (no padding, already >= width)
print(b"x".rjust(4))    # "   x"

# Test center
print(b"hi".center(6))  # "  hi  "
print(b"a".center(5))   # "  a  "
print(b"hello".center(3))  # "hello" (no padding)

# Test zfill
print(b"42".zfill(5))    # 00042
print(b"-42".zfill(5))   # -0042
print(b"hello".zfill(3)) # hello (no padding)

# Test strip methods
print(b"  hello  ".strip())   # "hello"
print(b"  hello  ".lstrip())  # "hello  "
print(b"  hello  ".rstrip())  # "  hello"
print(b"hello".strip())       # "hello" (no whitespace)

# Test islower
print(b"hello".islower())  # 1
print(b"Hello".islower())  # 0
print(b"HELLO".islower())  # 0

# Test isupper
print(b"HELLO".isupper())  # 1
print(b"Hello".isupper())  # 0
print(b"hello".isupper())  # 0

# Test isdigit
print(b"123".isdigit())    # 1
print(b"12a3".isdigit())   # 0
print(b"abc".isdigit())    # 0

# Test isalpha
print(b"abc".isalpha())    # 1
print(b"ab3c".isalpha())   # 0
print(b"123".isalpha())    # 0

# Test isalnum
print(b"abc123".isalnum()) # 1
print(b"abc".isalnum())    # 1
print(b"123".isalnum())    # 1

# Test isspace
print(b"   ".isspace())    # 1
print(b" a ".isspace())    # 0
print(b"".isspace())       # 0 (empty)

# Test find
print(b"hello".find(b"ll"))   # 2
print(b"hello".find(b"lo"))   # 3
print(b"hello".find(b"x"))    # -1 (not found)
print(b"hello".find(b""))     # 0 (empty string)

# Test count
print(b"banana".count(b"a"))   # 3
print(b"banana".count(b"na"))  # 2
print(b"hello".count(b"l"))    # 2
print(b"hello".count(b"x"))    # 0

# Test startswith
print(b"hello".startswith(b"he"))     # 1
print(b"hello".startswith(b"lo"))     # 0
print(b"hello".startswith(b""))       # 1 (empty)
print(b"hello".startswith(b"hello"))  # 1

# Test endswith
print(b"hello".endswith(b"lo"))       # 1
print(b"hello".endswith(b"he"))       # 0
print(b"hello".endswith(b""))         # 1 (empty)
print(b"hello".endswith(b"hello"))    # 1

# Test replace
print(b"hello".replace(b"l", b"L"))   # heLLo
print(b"banana".replace(b"a", b"o"))  # bonono
print(b"hello".replace(b"x", b"y"))   # hello (no match)

# Test chaining (method on method result)
print(b"hello".upper().lower())  # hello
print(b"  HELLO  ".strip().lower())  # hello
