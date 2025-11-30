# Test break in for loop
# expect: 0
# expect: 1
# expect: 2

for i in range(10):
    if i == 3:
        break
    print(i)
