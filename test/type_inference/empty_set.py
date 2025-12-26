# Type inference tests for empty sets
# These tests verify that empty set types can be inferred from usage
# NOTE: These tests will work once set support is fully implemented

def test_empty_set_add() -> int:
    """Empty set inferred from add() call"""
    print("Test: empty set with add")
    s = set()
    s.add(10)
    s.add(20)
    s.add(30)
    s.add(10)  # Duplicate, should not increase size

    size: int = len(s)
    print("Set size:", size)
    return size  # Expected: 3

def test_empty_set_contains() -> int:
    """Empty set with membership testing"""
    print("Test: empty set with contains")
    numbers = set()
    numbers.add(5)
    numbers.add(15)
    numbers.add(25)

    # Test membership (returns 1 for True, 0 for False in our type system)
    has_5: int = 1 if 5 in numbers else 0
    has_10: int = 1 if 10 in numbers else 0
    has_15: int = 1 if 15 in numbers else 0

    result: int = has_5 + has_10 + has_15
    print("Membership count:", result)
    return result  # Expected: 2 (5 and 15 are in set, 10 is not)

def test_empty_set_remove() -> int:
    """Empty set with remove operation"""
    print("Test: empty set with remove")
    items = set()
    items.add(100)
    items.add(200)
    items.add(300)

    size_before: int = len(items)
    print("Size before remove:", size_before)

    items.remove(200)

    size_after: int = len(items)
    print("Size after remove:", size_after)
    return size_after  # Expected: 2

def test_empty_set_iteration() -> int:
    """Iterate over empty set"""
    print("Test: empty set iteration")
    values = set()
    values.add(10)
    values.add(20)
    values.add(30)

    total: int = 0
    for value in values:
        total = total + value

    print("Total:", total)
    return total  # Expected: 60 (order may vary but sum is same)

def test_set_literal_non_empty() -> int:
    """Non-empty set literal (no inference needed)"""
    print("Test: set literal")
    s = {10, 20, 30, 40}

    size: int = len(s)
    print("Set size:", size)
    return size  # Expected: 4

def test_set_literal_duplicates() -> int:
    """Set literal with duplicates"""
    print("Test: set literal with duplicates")
    s = {1, 2, 3, 2, 1}  # Duplicates should be removed

    size: int = len(s)
    print("Set size after dedup:", size)
    return size  # Expected: 3

def test_set_add_duplicates() -> int:
    """Adding duplicates to set"""
    print("Test: set add duplicates")
    unique = set()

    # Add values with duplicates
    i: int = 0
    while i < 10:
        unique.add(i // 2)  # Will add 0, 0, 1, 1, 2, 2, 3, 3, 4, 4
        i = i + 1

    size: int = len(unique)
    print("Unique count:", size)
    return size  # Expected: 5 (values 0, 1, 2, 3, 4)

def test_set_membership_loop() -> int:
    """Check membership in a loop"""
    print("Test: set membership in loop")
    whitelist = set()
    whitelist.add(10)
    whitelist.add(20)
    whitelist.add(30)

    count: int = 0
    test_val: int = 0
    while test_val <= 30:
        if test_val in whitelist:
            count = count + 1
        test_val = test_val + 10

    print("Membership matches:", count)
    return count  # Expected: 3
