# Test bool operations with int (bool is subtype of int)
print(True & 1)   # 1
print(True | 1)   # 1
print(True ^ 1)   # 0
print(True == 1)  # True
print(False & 1)  # 0
print(False | 1)  # 1
print(False ^ 1)  # 1

# Cross-type comparison returns False
print(True == b"hello")   # False
print(True != b"hello")   # True
print(False == b"world")  # False
print(False != b"world")  # True
