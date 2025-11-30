# Test string % formatting operations

# Integer formatting
fmt_int: str = "Value: %d"
result_int: str = fmt_int % 42
print(result_int)

fmt_int2: str = "Negative: %d"
result_int2: str = fmt_int2 % -123
print(result_int2)

# Float formatting
fmt_float: str = "Float: %f"
result_float: str = fmt_float % 3.14
print(result_float)

fmt_float2: str = "Zero: %f"
result_float2: str = fmt_float2 % 0.0
print(result_float2)

# Bool formatting
fmt_bool: str = "Bool: %s"
result_bool1: str = fmt_bool % True
print(result_bool1)

result_bool2: str = fmt_bool % False
print(result_bool2)

# String formatting
fmt_str: str = "String: %s"
result_str: str = fmt_str % "hello"
print(result_str)

result_str2: str = fmt_str % "world"
print(result_str2)

# None formatting
fmt_none: str = "None: %s"
result_none: str = fmt_none % None
print(result_none)

# List formatting
fmt_list: str = "List: %s"
my_list: list[int] = [1, 2, 3]
result_list: str = fmt_list % my_list
print(result_list)

empty_list: list[int] = []
result_empty: str = fmt_list % empty_list
print(result_empty)

# Set formatting (with int elements only)
fmt_set: str = "Set: %s"
my_set: set[int] = {10, 20, 30}
result_set: str = fmt_set % my_set
print(len(result_set))  # Just check length since set order is non-deterministic

empty_set: set[int] = set()
result_empty_set: str = fmt_set % empty_set
print(result_empty_set)

# Bytes formatting
fmt_bytes: str = "Bytes: %s"
my_bytes: bytes = b"hello"
result_bytes: str = fmt_bytes % my_bytes
print(result_bytes)

print("All formatting tests passed!")
