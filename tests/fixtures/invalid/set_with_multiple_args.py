# Test set() builtin with multiple arguments
# Should fail: set() takes at most 1 argument

s: set[int] = set(1, 2, 3)
