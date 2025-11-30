# Test for loop over strings

# Iterate over string characters
text: str = "hello"
for char in text:
    print(b"Char:", char)

# Count vowels
vowels: str = "aeiou"
count: int = 0
for c in "hello world":
    if c in vowels:
        count = count + 1
print(b"Vowel count:", count)

# Build string character by character
result: str = ""
for c in "abc":
    result = result + c + "-"
print(b"Result:", result)

# Iterate over bytes
data: bytes = b"hello"
for b in data:
    print(b"Byte:", b)
