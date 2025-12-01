# String formatting with % operator for all supported types
s1: str = "Value: %d" % 42
print(s1)

s2: str = "Float: %f" % 3.14159
print(s2)

s3: str = "Bool: %s" % True
print(s3)

s4: str = "Name: %s" % "Alice"
print(s4)

s5: str = "List: %s" % [1, 2, 3]
print(s5)

s6: str = "Set: %s" % {1, 2, 3}
print(s6)
