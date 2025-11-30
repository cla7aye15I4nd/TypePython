# Test comprehensive string iteration patterns

# Empty string
empty: str = ""
for c in empty:
    print(b"Never")
print(b"Empty string done")

# Single char
single: str = "x"
for c in single:
    print(b"Single:", c)

# Character types
text: str = "Hello, World! 123"
letters: int = 0
digits: int = 0
spaces: int = 0
other: int = 0
for c in text:
    if c.isalpha():
        letters = letters + 1
    elif c.isdigit():
        digits = digits + 1
    elif c.isspace():
        spaces = spaces + 1
    else:
        other = other + 1
print(b"Letters:", letters)
print(b"Digits:", digits)
print(b"Spaces:", spaces)
print(b"Other:", other)

# Build character list
chars: list[str] = []
for c in "python":
    chars.append(c)
print(b"Chars:", chars)

# Reverse string
original: str = "hello"
reversed_s: str = ""
for c in original:
    reversed_s = c + reversed_s
print(b"Reversed:", reversed_s)

# Character frequency
freq: dict[str, int] = {}
for c in "mississippi":
    if c in freq:
        freq[c] = freq[c] + 1
    else:
        freq[c] = 1
print(b"Frequency:", freq)

# Find positions
text2: str = "banana"
a_positions: list[int] = []
for i, c in enumerate(text2):
    if c == "a":
        a_positions.append(i)
print(b"'a' positions:", a_positions)

# Transform characters
upper_list: list[str] = []
for c in "hello":
    upper_list.append(c.upper())
print(b"Upper:", upper_list)

# Filter characters
alpha_only: str = ""
for c in "abc123def456":
    if c.isalpha():
        alpha_only = alpha_only + c
print(b"Alpha only:", alpha_only)

# Check conditions
all_lower: bool = True
for c in "hello":
    if not c.islower():
        all_lower = False
print(b"All lower:", all_lower)

any_upper: bool = False
for c in "helloWorld":
    if c.isupper():
        any_upper = True
        break
print(b"Any upper:", any_upper)

# Word iteration
sentence: str = "the quick brown fox"
for word in sentence.split():
    print(b"Word:", word)

# Line iteration
multiline: str = "line1\nline2\nline3"
for line in multiline.split("\n"):
    print(b"Line:", line)

# Character pairs
s: str = "abcdef"
for i in range(len(s) - 1):
    print(b"Pair:", s[i], s[i + 1])

# Sliding window
for i in range(len(s) - 2):
    window: str = s[i:i + 3]
    print(b"Window:", window)

# Palindrome check
def is_palindrome(s: str) -> bool:
    for i in range(len(s) // 2):
        if s[i] != s[len(s) - 1 - i]:
            return False
    return True

test_words: list[str] = ["radar", "hello", "level", "python"]
for word in test_words:
    print(b"Palindrome", word, is_palindrome(word))

# Anagram check via sorting
def sorted_chars(s: str) -> str:
    chars: list[str] = []
    for c in s:
        chars.append(c)
    chars.sort()
    result: str = ""
    for c in chars:
        result = result + c
    return result

print(b"Sorted 'listen':", sorted_chars("listen"))
print(b"Sorted 'silent':", sorted_chars("silent"))

# Caesar cipher
def caesar(s: str, shift: int) -> str:
    result: str = ""
    for c in s:
        if c.isalpha():
            base: int = ord("a") if c.islower() else ord("A")
            shifted: int = (ord(c) - base + shift) % 26 + base
            result = result + chr(shifted)
        else:
            result = result + c
    return result

print(b"Caesar 'hello' +3:", caesar("hello", 3))

# Run-length encoding
def rle(s: str) -> str:
    if len(s) == 0:
        return ""
    result: str = ""
    current: str = s[0]
    count: int = 1
    for i in range(1, len(s)):
        if s[i] == current:
            count = count + 1
        else:
            result = result + current + str(count)
            current = s[i]
            count = 1
    result = result + current + str(count)
    return result

print(b"RLE 'aaabbbcc':", rle("aaabbbcc"))
