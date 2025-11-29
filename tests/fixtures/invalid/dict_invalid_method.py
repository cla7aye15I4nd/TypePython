# Test invalid dict method
# Should fail: dict has no method 'nonexistent'

my_dict: dict[int, int] = {1: 10, 2: 20}
my_dict.nonexistent()
