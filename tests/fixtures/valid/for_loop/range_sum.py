# Test using for loop to compute sum
# expect: 55

total: int = 0
for i in range(1, 11):
    total = total + i

print(total)
