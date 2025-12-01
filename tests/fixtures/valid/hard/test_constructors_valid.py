# Test valid usage of dict(), set(), and list() constructors

# ============================================================================
# dict() tests
# ============================================================================

# Create empty dict
empty_dict: dict[int, int] = dict()
print(len(empty_dict))

# Add items to dict
empty_dict[1] = 10
empty_dict[2] = 20
print(len(empty_dict))
print(empty_dict[1])
print(empty_dict[2])

# Create dict using literal and check it works
literal_dict: dict[int, int] = {3: 30, 4: 40}
print(len(literal_dict))
print(literal_dict[3])
print(literal_dict[4])

# ============================================================================
# set() tests
# ============================================================================

# Create empty set
empty_set: set[int] = set()
print(len(empty_set))

# Add items to set
empty_set.add(1)
empty_set.add(2)
empty_set.add(3)
print(len(empty_set))
print(1 in empty_set)
print(2 in empty_set)
print(3 in empty_set)

# Create set using literal
literal_set: set[int] = {10, 20, 30}
print(len(literal_set))
print(10 in literal_set)

# Copy a set using set()
original_set: set[int] = {100, 200, 300}
copied_set: set[int] = set(original_set)
print(len(copied_set))

# Verify copy is independent
copied_set.add(400)
print(len(original_set))  # Should be 3
print(len(copied_set))    # Should be 4
print(400 in original_set)  # Should be False
print(400 in copied_set)    # Should be True

# ============================================================================
# list() tests
# ============================================================================

# Create empty list
empty_list: list[int] = list()
print(len(empty_list))

# Append items to list
empty_list.append(1)
empty_list.append(2)
empty_list.append(3)
print(len(empty_list))
print(empty_list[0])
print(empty_list[1])
print(empty_list[2])

# Create list using literal
literal_list: list[int] = [10, 20, 30]
print(len(literal_list))
print(literal_list)

# ============================================================================
# Mixed usage
# ============================================================================

# Use constructors in expressions
new_dict: dict[int, int] = dict()
new_set: set[int] = set()
new_list: list[int] = list()

new_dict[5] = 50
new_set.add(5)
new_list.append(5)

print(new_dict[5])
print(5 in new_set)
print(new_list[0])

# Use with len()
print(len(new_dict))
print(len(new_set))
print(len(new_list))
