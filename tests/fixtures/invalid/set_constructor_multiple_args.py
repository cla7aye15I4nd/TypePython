# set() should accept at most 1 argument
# Expected error: "set() takes at most 1 argument"

s: set[int] = set({1, 2}, {3, 4})
