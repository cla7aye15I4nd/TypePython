# Test set() builtin with wrong argument type
# Should fail: set() argument must be a set

s: set[int] = set(42)
