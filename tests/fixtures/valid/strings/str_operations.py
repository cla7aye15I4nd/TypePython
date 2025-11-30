# Test string operations

# Concatenation
s1: str = "hello"
s2: str = "world"
print(s1 + " " + s2)

# Repetition
print("ab" * 3)
print("x" * 5)
print("hi" * True)
print("test" * False)

# Comparison
print("abc" == "abc")
print("abc" == "xyz")
print("abc" != "xyz")
print("abc" != "abc")

# Ordering
print("apple" < "banana")
print("banana" < "apple")
print("apple" <= "apple")
print("apple" <= "banana")
print("banana" > "apple")
print("apple" > "banana")
print("apple" >= "apple")
print("banana" >= "apple")

# In operator (substring)
print("ell" in "hello")
print("xyz" in "hello")
print("ell" not in "hello")
print("xyz" not in "hello")

