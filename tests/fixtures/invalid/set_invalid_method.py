# Test invalid set method
# Should fail: set has no method 'nonexistent'

my_set: set[int] = {1, 2, 3}
my_set.nonexistent()
