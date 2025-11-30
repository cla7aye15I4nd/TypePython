# Test iterating over a list
# expect: 10
# expect: 20
# expect: 30

nums: list[int] = [10, 20, 30]
for x in nums:
    print(x)
