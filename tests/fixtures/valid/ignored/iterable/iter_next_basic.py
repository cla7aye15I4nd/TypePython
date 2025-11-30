# Test iter() and next() builtins

# iter on list and next
nums: list[int] = [1, 2, 3, 4, 5]
it = iter(nums)
print(b"First:", next(it))
print(b"Second:", next(it))
print(b"Third:", next(it))

# next with default value
it2 = iter([10, 20])
print(b"Val:", next(it2))
print(b"Val:", next(it2))
print(b"Default:", next(it2, -1))

# iter on string
s_it = iter("hello")
print(b"Char:", next(s_it))
print(b"Char:", next(s_it))

# Manual iteration pattern
nums2: list[int] = [100, 200, 300]
iterator = iter(nums2)
done: bool = False
while not done:
    val: int = next(iterator, -999)
    if val == -999:
        done = True
    else:
        print(b"Value:", val)

# iter on range
r_it = iter(range(3))
print(b"Range val:", next(r_it))
print(b"Range val:", next(r_it))
print(b"Range val:", next(r_it))
