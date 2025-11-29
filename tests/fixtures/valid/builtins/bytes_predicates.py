# Test bytes predicate methods: islower, isupper, isalpha, isdigit, isalnum, isspace

# Test islower
print(b"hello".islower())      # True
print(b"HELLO".islower())      # False
print(b"Hello".islower())      # False
print(b"hello123".islower())   # True (digits don't affect)
print(b"".islower())           # False (empty)

# Test isupper
print(b"HELLO".isupper())      # True
print(b"hello".isupper())      # False
print(b"Hello".isupper())      # False
print(b"WORLD456".isupper())   # True (digits don't affect)
print(b"".isupper())           # False (empty)

# Test isalpha
print(b"hello".isalpha())      # True
print(b"WORLD".isalpha())      # True
print(b"Hello123".isalpha())   # False (has digits)
print(b"test!".isalpha())      # False (has special char)
print(b"".isalpha())           # False (empty)

# Test isdigit
print(b"123".isdigit())        # True
print(b"456".isdigit())        # True
print(b"12a".isdigit())        # False (has letter)
print(b"".isdigit())           # False (empty)
print(b"0".isdigit())          # True

# Test isalnum
print(b"hello123".isalnum())   # True
print(b"TEST456".isalnum())    # True
print(b"abc".isalnum())        # True
print(b"123".isalnum())        # True
print(b"test!123".isalnum())   # False (has special char)
print(b"".isalnum())           # False (empty)

# Test isspace
print(b" ".isspace())          # True
print(b"  ".isspace())         # True
print(b" a ".isspace())        # False (has letter)
print(b"".isspace())           # False (empty)
print(b"hello".isspace())      # False

# Mixed tests
print(b"abc".islower())        # True
print(b"ABC".isupper())        # True
print(b"aBc".islower())        # False
print(b"aBc".isupper())        # False

# Edge cases
print(b"1".isdigit())          # True
print(b"a".isalpha())          # True
print(b"A".isalpha())          # True
print(b"0123456789".isdigit()) # True
