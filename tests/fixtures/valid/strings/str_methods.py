# Test string methods

# Case conversion methods
s: str = "hello WORLD"
print(s.upper())
print(s.lower())
print(s.capitalize())
print(s.title())
print(s.swapcase())

# Strip methods
padded: str = "  hello  "
print(padded.strip())
print(padded.lstrip())
print(padded.rstrip())

# Search methods
text: str = "hello world hello"
print(text.find("world"))
print(text.find("xyz"))
print(text.count("hello"))
print(text.count("l"))

# Predicate methods
print("hello".startswith("hel"))
print("hello".startswith("xyz"))
print("hello".endswith("llo"))
print("hello".endswith("xyz"))
print("abc123".isalnum())
print("abc".isalpha())
print("123".isdigit())
print("   ".isspace())
print("hello".islower())
print("HELLO".isupper())

# Replace method
original: str = "hello world"
print(original.replace("world", "python"))

# Padding methods
word: str = "hi"
print(word.ljust(5))
print(word.rjust(5))
print(word.center(6))
print(word.zfill(5))
