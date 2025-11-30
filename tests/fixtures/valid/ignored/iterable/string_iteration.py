# Test comprehensive string iteration patterns

# Character iteration
text: str = "Hello, World!"
for c in text:
    print(b"Char:", c)

# Count specific character
count: int = 0
for c in "mississippi":
    if c == "s":
        count = count + 1
print(b"Count of s:", count)

# Build reversed string
original: str = "python"
reversed_str: str = ""
for c in original:
    reversed_str = c + reversed_str
print(b"Reversed:", reversed_str)

# Find character positions
text2: str = "banana"
positions: list[int] = []
for i, c in enumerate(text2):
    if c == "a":
        positions.append(i)
print(b"Positions of a:", positions)

# Character transformation
upper_chars: list[str] = []
for c in "hello":
    upper_chars.append(c.upper())
print(b"Upper chars:", upper_chars)

# Check all characters
all_alpha: bool = True
for c in "hello123":
    if not c.isalpha():
        all_alpha = False
        break
print(b"All alpha:", all_alpha)

# Split and iterate
sentence: str = "the quick brown fox"
words: list[str] = sentence.split()
for word in words:
    print(b"Word:", word)

# Iterate lines
multiline: str = "line1\nline2\nline3"
for line in multiline.split("\n"):
    print(b"Line:", line)

# Character frequency
freq: dict[str, int] = {}
for c in "abracadabra":
    freq[c] = freq.get(c, 0) + 1
print(b"Frequency:", freq)

# Palindrome check via iteration
def is_palindrome(s: str) -> bool:
    i: int = 0
    j: int = len(s) - 1
    while i < j:
        if s[i] != s[j]:
            return False
        i = i + 1
        j = j - 1
    return True

for word in ["radar", "hello", "level", "world"]:
    print(b"Palindrome", word, b":", is_palindrome(word))

# Iterate with index
text3: str = "python"
for i in range(len(text3)):
    print(b"Index", i, b":", text3[i])
