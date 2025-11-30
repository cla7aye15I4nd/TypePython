# Test nested for loops
# expect: 0 0
# expect: 0 1
# expect: 1 0
# expect: 1 1
# expect: 2 0
# expect: 2 1

for i in range(3):
    for j in range(2):
        print(i, j)
