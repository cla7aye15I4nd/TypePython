# Test continue in for loop
# expect: 1
# expect: 3
# expect: 5
# expect: 7
# expect: 9

for i in range(10):
    if i % 2 == 0:
        continue
    print(i)
