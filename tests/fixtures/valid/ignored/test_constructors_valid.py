# Test valid usage of dict(), set(), and list() constructors

# ============================================================================
# dict() tests
# ============================================================================

# Create empty dict
empty_dict: dict[int, int] = dict()
print(empty_dict)

# Add items to dict
empty_dict[1] = 10
empty_dict[2] = 20
print(empty_dict)

# Create dict using literal and check it works
literal_dict: dict[int, int] = {3: 30, 4: 40}
print(literal_dict)

# ============================================================================
# set() tests
# ============================================================================

# Create empty set
empty_set: set[int] = set()
print(empty_set)

# Add items to set
empty_set.add(1)
empty_set.add(2)
empty_set.add(3)
print(empty_set)

# Create set using literal
literal_set: set[int] = {10, 20, 30}
print(literal_set)

# Copy a set using set()
original_set: set[int] = {100, 200, 300}
copied_set: set[int] = set(original_set)
print(copied_set)

# Verify copy is independent
copied_set.add(400)
print(original_set)  # Should not have 400
print(copied_set)    # Should have 400

# ============================================================================
# list() tests
# ============================================================================

# Create empty list
empty_list: list[int] = list()
print(empty_list)

# Append items to list
empty_list.append(1)
empty_list.append(2)
empty_list.append(3)
print(empty_list)

# Create list using literal
literal_list: list[int] = [10, 20, 30]
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

print(new_dict)
print(new_set)
print(new_list)

# Use with len()
print(len(empty_dict))
print(len(empty_set))
print(len(empty_list))
