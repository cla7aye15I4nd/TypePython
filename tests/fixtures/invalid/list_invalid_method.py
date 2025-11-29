# Test invalid list method
# Should fail: list has no method 'nonexistent'

my_list: list[int] = [1, 2, 3]
my_list.nonexistent()
