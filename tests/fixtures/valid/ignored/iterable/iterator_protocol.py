# Test iterator protocol edge cases

# Exhausted iterator
nums: list[int] = [1, 2, 3]
it = iter(nums)
print(b"1:", next(it))
print(b"2:", next(it))
print(b"3:", next(it))
print(b"Exhausted default:", next(it, -1))
print(b"Still exhausted:", next(it, -1))

# Multiple iterators on same collection
data: list[int] = [10, 20, 30]
it1 = iter(data)
it2 = iter(data)
print(b"it1:", next(it1))
print(b"it2:", next(it2))
print(b"it1:", next(it1))
print(b"it2:", next(it2))

# iter on already an iterator
nums2: list[int] = [1, 2, 3]
it3 = iter(nums2)
it4 = iter(it3)  # Returns same iterator
print(b"Same?:", it3 is it4)

# Iterator in boolean context
it5 = iter([1, 2, 3])
if it5:
    print(b"Iterator is truthy")

# Partial iteration then loop
partial: list[int] = [1, 2, 3, 4, 5]
it6 = iter(partial)
print(b"First:", next(it6))
print(b"Second:", next(it6))
for remaining in it6:
    print(b"Remaining:", remaining)

# next with various default types
it7 = iter([])
print(b"Default int:", next(it7, 0))
it8 = iter([])
print(b"Default str:", next(it8, "none"))
it9 = iter([])
print(b"Default None:", next(it9, None))
