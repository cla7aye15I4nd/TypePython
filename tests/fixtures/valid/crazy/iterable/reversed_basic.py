# Test reversed() builtin

# reversed on list
nums: list[int] = [1, 2, 3, 4, 5]
for n in reversed(nums):
    print(b"Reversed:", n)

# reversed on string
text: str = "hello"
for c in reversed(text):
    print(b"Char:", c)

# reversed on range
for i in reversed(range(5)):
    print(b"Rev range:", i)

# Collect reversed into list
original: list[str] = ["a", "b", "c"]
rev_list: list[str] = []
for item in reversed(original):
    rev_list.append(item)
print(b"Reversed list:", rev_list)

# reversed with step in range
for i in reversed(range(0, 10, 2)):
    print(b"Rev evens:", i)
