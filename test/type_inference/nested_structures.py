# Complex type inference tests: Nested structures
# These tests verify type inference for nested containers

def test_nested_list_basic() -> int:
    """Nested list: list[list[int]]"""
    print("Test: nested list basic")
    outer = []
    inner = []

    inner.append(10)
    inner.append(20)

    outer.append(inner)

    # Access nested element: outer[0][1] should be 20
    result: int = outer[0][1]
    print("Nested value:", result)
    return result  # Expected: 20

def test_nested_list_multiple_inner() -> int:
    """Multiple inner lists in outer list"""
    print("Test: multiple inner lists")
    matrix = []

    row1 = []
    row1.append(1)
    row1.append(2)

    row2 = []
    row2.append(3)
    row2.append(4)

    matrix.append(row1)
    matrix.append(row2)

    # Sum all elements
    total: int = matrix[0][0] + matrix[0][1] + matrix[1][0] + matrix[1][1]
    print("Matrix sum:", total)
    return total  # Expected: 10

def test_triple_nested() -> int:
    """Triple nested list: list[list[list[int]]]"""
    print("Test: triple nested")
    level1 = []
    level2 = []
    level3 = []

    level3.append(42)
    level2.append(level3)
    level1.append(level2)

    result: int = level1[0][0][0]
    print("Triple nested value:", result)
    return result  # Expected: 42

def test_nested_dict_in_list() -> int:
    """List of dicts: list[dict[int, int]]"""
    print("Test: list of dicts")
    records = []

    record1 = {}
    record1[1] = 100
    record1[2] = 200

    record2 = {}
    record2[1] = 300
    record2[2] = 400

    records.append(record1)
    records.append(record2)

    # Access: records[1][2] should be 400
    result: int = records[1][2]
    print("Nested dict value:", result)
    return result  # Expected: 400

def test_nested_dict_basic() -> int:
    """Nested dict: dict[int, dict[int, int]]"""
    print("Test: nested dict basic")
    outer_dict = {}
    inner_dict = {}

    inner_dict[10] = 100
    inner_dict[20] = 200

    outer_dict[1] = inner_dict

    # Access: outer_dict[1][10] should be 100
    result: int = outer_dict[1][10]
    print("Nested dict value:", result)
    return result  # Expected: 100

def test_nested_dict_multiple() -> int:
    """Multiple nested dicts"""
    print("Test: multiple nested dicts")
    users = {}

    alice_scores = {}
    alice_scores[1] = 90
    alice_scores[2] = 85

    bob_scores = {}
    bob_scores[1] = 75
    bob_scores[2] = 95

    users[1] = alice_scores
    users[2] = bob_scores

    # Sum all scores
    total: int = users[1][1] + users[1][2] + users[2][1] + users[2][2]
    print("Total scores:", total)
    return total  # Expected: 345

def test_list_in_dict() -> int:
    """Dict of lists: dict[int, list[int]]"""
    print("Test: dict of lists")
    groups = {}

    group1 = []
    group1.append(10)
    group1.append(20)

    group2 = []
    group2.append(30)
    group2.append(40)

    groups[1] = group1
    groups[2] = group2

    # Access: groups[2][1] should be 40
    result: int = groups[2][1]
    print("Value:", result)
    return result  # Expected: 40

def test_set_in_dict() -> int:
    """Dict of sets: dict[int, set[int]]"""
    print("Test: dict of sets")
    collections = {}

    set1 = set()
    set1.add(10)
    set1.add(20)

    set2 = set()
    set2.add(30)
    set2.add(40)

    collections[1] = set1
    collections[2] = set2

    # Check membership and count
    has_20: int = 1 if 20 in collections[1] else 0
    has_30: int = 1 if 30 in collections[2] else 0
    has_50: int = 1 if 50 in collections[2] else 0

    result: int = has_20 + has_30 + has_50
    print("Membership count:", result)
    return result  # Expected: 2
