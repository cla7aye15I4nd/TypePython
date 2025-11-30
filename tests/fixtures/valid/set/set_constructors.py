# Test set() constructor variations

# Empty set
empty: set[int] = set()
print(len(empty))

# Set from existing set (copy)
original: set[int] = {1, 2, 3}
copied: set[int] = set(original)
print(len(copied))
print(1 in copied)

# Set from string
char_set: set[str] = set("hello")
print(len(char_set))

# Set from bytes
byte_set: set[int] = set(b"hello")
print(len(byte_set))

# Set from list
list_set: set[int] = set([1, 2, 2, 3, 3, 3])
print(len(list_set))
print(1 in list_set)
print(2 in list_set)
print(3 in list_set)

# Set from dict (keys only)
d: dict[int, int] = {1: 10, 2: 20, 3: 30}
dict_set: set[int] = set(d)
print(len(dict_set))
print(1 in dict_set)
print(2 in dict_set)
