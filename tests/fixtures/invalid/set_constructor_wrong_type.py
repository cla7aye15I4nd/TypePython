# set() with argument that is not a set should fail
# Expected error: "set() argument must be a set"

s: set[int] = set(123)
