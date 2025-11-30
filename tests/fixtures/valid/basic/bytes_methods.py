# Test bytes methods

# Case conversion methods
b: bytes = b"hello WORLD"
print(b.upper())
print(b.lower())
print(b.capitalize())
print(b.title())
print(b.swapcase())

# Strip methods
padded: bytes = b"  hello  "
print(padded.strip())
print(padded.lstrip())
print(padded.rstrip())

# Search methods
text: bytes = b"hello world hello"
print(text.find(b"world"))
print(text.find(b"xyz"))
print(text.count(b"hello"))
print(text.count(b"l"))

# Predicate methods
print(b"hello".startswith(b"hel"))
print(b"hello".startswith(b"xyz"))
print(b"hello".endswith(b"llo"))
print(b"hello".endswith(b"xyz"))
print(b"abc123".isalnum())
print(b"abc".isalpha())
print(b"123".isdigit())
print(b"   ".isspace())
print(b"hello".islower())
print(b"HELLO".isupper())

# Replace method
original: bytes = b"hello world"
print(original.replace(b"world", b"python"))

# Padding methods
word: bytes = b"hi"
print(word.ljust(5))
print(word.rjust(5))
print(word.center(6))
print(word.zfill(5))
