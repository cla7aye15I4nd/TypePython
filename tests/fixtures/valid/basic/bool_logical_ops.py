# Test boolean logical operators (and/or) at the bool type level
# These test BoolType.binary_op_impl with And/Or operators

# Boolean and operation (direct bool and bool)
a: bool = True
b: bool = False

# Using bitwise AND/OR which go through BinaryOp::And/Or for bools
# Note: In Python, 'and'/'or' on bools use short-circuit evaluation,
# but for this type system test, we use the bitwise operators which
# map to BinaryOp::And/Or for bool types

# Direct bool binary and/or
print(b"Bool And tests:")
r1: bool = True and True
r2: bool = True and False
r3: bool = False and True
r4: bool = False and False
print(r1)
print(r2)
print(r3)
print(r4)

print(b"Bool Or tests:")
r5: bool = True or True
r6: bool = True or False
r7: bool = False or True
r8: bool = False or False
print(r5)
print(r6)
print(r7)
print(r8)
