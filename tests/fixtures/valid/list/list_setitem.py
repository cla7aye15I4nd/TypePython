# List setitem
nums: list[int] = [1, 2, 3, 4, 5]

print(nums[2])
nums[2] = 99
print(nums[2])

# Verify other elements unchanged
print(nums[0])
print(nums[4])
