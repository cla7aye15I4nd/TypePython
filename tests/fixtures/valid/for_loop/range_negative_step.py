# Test range with negative step
# expect: 10
# expect: 8
# expect: 6
# expect: 4
# expect: 2

for i in range(10, 0, -2):
    print(i)
