# Test bytes padding methods: ljust, rjust, center, zfill

# Test ljust (left justify, pad right)
print(b"hello".ljust(10))      # b'hello     '
print(b"test".ljust(10))       # b'test      '
print(b"hi".ljust(5))          # b'hi   '
print(b"toolong".ljust(3))     # b'toolong' (no truncation)
print(b"exact".ljust(5))       # b'exact'

# Test rjust (right justify, pad left)
print(b"hello".rjust(10))      # b'     hello'
print(b"test".rjust(10))       # b'      test'
print(b"hi".rjust(5))          # b'   hi'
print(b"toolong".rjust(3))     # b'toolong' (no truncation)
print(b"exact".rjust(5))       # b'exact'

# Test center (center, pad both sides)
print(b"hello".center(11))     # b'   hello   '
print(b"test".center(10))      # b'   test   '
print(b"hi".center(6))         # b'  hi  '
print(b"odd".center(8))        # b'  odd   ' or b'   odd  '
print(b"toolong".center(3))    # b'toolong' (no truncation)

# Test zfill (zero-fill)
print(b"42".zfill(5))          # b'00042'
print(b"123".zfill(6))         # b'000123'
print(b"7".zfill(3))           # b'007'
print(b"toolong".zfill(3))     # b'toolong' (no truncation)
print(b"99".zfill(2))          # b'99'

# Edge cases
print(b"".ljust(5))            # b'     '
print(b"".rjust(5))            # b'     '
print(b"".center(5))           # b'     ' or similar
print(b"".zfill(3))            # b'000'

# Zero or negative width
print(b"test".ljust(0))        # b'test'
print(b"test".rjust(0))        # b'test'
print(b"test".center(0))       # b'test'
print(b"test".zfill(0))        # b'test'

# Single character
print(b"x".ljust(5))           # b'x    '
print(b"x".rjust(5))           # b'    x'
print(b"x".center(5))          # b'  x  '
print(b"x".zfill(5))           # b'0000x'
