# Test bytes methods not covered elsewhere:
# capitalize, title, swapcase, ljust, rjust, center, zfill,
# strip, lstrip, rstrip, find, count, startswith, endswith, replace

# ============================================================================
# 1. capitalize - first char uppercase, rest lowercase
# ============================================================================
print(b"1. capitalize:")
print(b"hello".capitalize())
print(b"HELLO".capitalize())
print(b"hELLO wORLD".capitalize())
print(b"".capitalize())
print(b"a".capitalize())
print(b"123abc".capitalize())

# ============================================================================
# 2. title - first char of each word uppercase
# ============================================================================
print(b"2. title:")
print(b"hello world".title())
print(b"HELLO WORLD".title())
print(b"hello there".title())
print(b"".title())
print(b"a b c".title())

# ============================================================================
# 3. swapcase - swap upper/lower
# ============================================================================
print(b"3. swapcase:")
print(b"Hello World".swapcase())
print(b"hELLO wORLD".swapcase())
print(b"UPPER".swapcase())
print(b"lower".swapcase())
print(b"".swapcase())
print(b"123abc".swapcase())

# ============================================================================
# 4. ljust - left justify, pad with spaces
# ============================================================================
print(b"4. ljust:")
print(b"hi".ljust(5))
print(b"hello".ljust(10))
print(b"test".ljust(4))
print(b"".ljust(3))
print(b"longer".ljust(3))

# ============================================================================
# 5. rjust - right justify, pad with spaces
# ============================================================================
print(b"5. rjust:")
print(b"hi".rjust(5))
print(b"hello".rjust(10))
print(b"test".rjust(4))
print(b"".rjust(3))
print(b"longer".rjust(3))

# ============================================================================
# 6. center - center, pad with spaces
# ============================================================================
print(b"6. center:")
print(b"hi".center(6))
print(b"abc".center(7))
print(b"test".center(4))
print(b"".center(4))
print(b"longer".center(3))

# ============================================================================
# 7. zfill - pad with zeros on left
# ============================================================================
print(b"7. zfill:")
print(b"42".zfill(5))
print(b"-42".zfill(5))
print(b"abc".zfill(6))
print(b"".zfill(3))
print(b"12345".zfill(3))

# ============================================================================
# 8. strip - remove leading/trailing whitespace
# ============================================================================
print(b"8. strip:")
print(b"  hello  ".strip())
print(b"hello".strip())
print(b"   ".strip())
print(b"".strip())
print(b"\t\nhello\n\t".strip())

# ============================================================================
# 9. lstrip - remove leading whitespace
# ============================================================================
print(b"9. lstrip:")
print(b"  hello  ".lstrip())
print(b"hello".lstrip())
print(b"   ".lstrip())
print(b"".lstrip())
print(b"\t\nhello".lstrip())

# ============================================================================
# 10. rstrip - remove trailing whitespace
# ============================================================================
print(b"10. rstrip:")
print(b"  hello  ".rstrip())
print(b"hello".rstrip())
print(b"   ".rstrip())
print(b"".rstrip())
print(b"hello\n\t".rstrip())

# ============================================================================
# 11. find - find substring, return index or -1
# ============================================================================
print(b"11. find:")
print(b"hello world".find(b"world"))
print(b"hello world".find(b"o"))
print(b"hello world".find(b"xyz"))
print(b"hello".find(b""))
print(b"".find(b"a"))
print(b"banana".find(b"an"))

# ============================================================================
# 12. count - count occurrences
# ============================================================================
print(b"12. count:")
print(b"hello world".count(b"o"))
print(b"banana".count(b"an"))
print(b"hello".count(b"l"))
print(b"hello".count(b"xyz"))
print(b"".count(b"a"))
print(b"aaa".count(b"a"))

# ============================================================================
# 13. startswith - check prefix
# ============================================================================
print(b"13. startswith:")
print(b"hello world".startswith(b"hello"))
print(b"hello world".startswith(b"world"))
print(b"hello".startswith(b""))
print(b"".startswith(b""))
print(b"test".startswith(b"test"))
print(b"test".startswith(b"testing"))

# ============================================================================
# 14. endswith - check suffix
# ============================================================================
print(b"14. endswith:")
print(b"hello world".endswith(b"world"))
print(b"hello world".endswith(b"hello"))
print(b"hello".endswith(b""))
print(b"".endswith(b""))
print(b"test".endswith(b"test"))
print(b"test".endswith(b"testing"))

# ============================================================================
# 15. replace - replace substring
# ============================================================================
print(b"15. replace:")
print(b"hello world".replace(b"world", b"python"))
print(b"aaa".replace(b"a", b"b"))
print(b"hello".replace(b"x", b"y"))
print(b"hello".replace(b"l", b"L"))
print(b"".replace(b"a", b"b"))
print(b"abcabc".replace(b"abc", b"x"))
