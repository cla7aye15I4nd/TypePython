# Complex type inference tests: Conditional branches
# These tests verify type inference across if/else branches

def test_both_branches_same_type() -> int:
    """Both branches append same type"""
    print("Test: both branches same type")
    result = []

    condition: int = 1

    if condition:
        result.append(10)
    else:
        result.append(20)

    # result: list[int] (unified from both branches)
    value: int = result[0]
    print("Value:", value)
    return value  # Expected: 10

def test_nested_conditionals() -> int:
    """Nested if/else with type inference"""
    print("Test: nested conditionals")
    data = []

    outer_cond: int = 1
    inner_cond: int = 0

    if outer_cond:
        if inner_cond:
            data.append(100)
        else:
            data.append(200)
    else:
        data.append(300)

    # data: list[int]
    result: int = data[0]
    print("Result:", result)
    return result  # Expected: 200

def test_multiple_branches_building_dict() -> int:
    """Multiple branches building dict"""
    print("Test: multiple branches dict")
    config = {}

    mode: int = 2

    if mode == 1:
        config[1] = 100
    elif mode == 2:
        config[2] = 200
    else:
        config[3] = 300

    # config: dict[int, int]
    result: int = config[2]
    print("Config:", result)
    return result  # Expected: 200

def test_conditional_with_different_operations() -> int:
    """Different operations in each branch but same type"""
    print("Test: conditional different operations")
    numbers = []

    flag: int = 1

    if flag:
        # Build via loop in one branch
        i: int = 0
        while i < 3:
            numbers.append(i)
            i = i + 1
    else:
        # Direct appends in other branch
        numbers.append(10)
        numbers.append(20)
        numbers.append(30)

    # numbers: list[int]
    size: int = len(numbers)
    print("Size:", size)
    return size  # Expected: 3

def test_early_return_with_inference() -> int:
    """Early return doesn't break type inference"""
    print("Test: early return")
    values = []

    special_case: int = 0

    if special_case:
        return 999

    # Continue building values
    values.append(1)
    values.append(2)
    values.append(3)

    result: int = values[1]
    print("Result:", result)
    return result  # Expected: 2

def test_conditional_append_to_nested() -> int:
    """Conditional branches appending to nested structure"""
    print("Test: conditional nested append")
    matrix = []
    row1 = []
    row2 = []

    which_row: int = 1

    if which_row == 1:
        row1.append(10)
        row1.append(20)
        matrix.append(row1)
    else:
        row2.append(30)
        row2.append(40)
        matrix.append(row2)

    # matrix: list[list[int]]
    result: int = matrix[0][0]
    print("Matrix value:", result)
    return result  # Expected: 10

def test_conditional_dict_update() -> int:
    """Update dict values conditionally"""
    print("Test: conditional dict update")
    scores = {}
    scores[1] = 50

    improved: int = 1

    if improved:
        # Update existing key
        scores[1] = 100
        # Add new key
        scores[2] = 90
    else:
        scores[1] = 60

    # scores: dict[int, int]
    result: int = scores[1]
    print("Updated score:", result)
    return result  # Expected: 100

def test_conditional_set_operations() -> int:
    """Set operations in conditional branches"""
    print("Test: conditional set operations")
    allowed = set()

    permission_level: int = 2

    if permission_level == 1:
        allowed.add(10)
    elif permission_level == 2:
        allowed.add(20)
        allowed.add(30)
    else:
        allowed.add(40)

    # allowed: set[int]
    has_30: int = 1 if 30 in allowed else 0
    print("Has 30:", has_30)
    return has_30  # Expected: 1

def test_guard_clauses() -> int:
    """Multiple guard clauses with early returns"""
    print("Test: guard clauses")
    data = []

    # Guard clause 1
    error_case: int = 0
    if error_case:
        return -1

    # Guard clause 2
    special_case: int = 0
    if special_case:
        return -2

    # Normal flow
    data.append(100)
    data.append(200)

    result: int = data[0] + data[1]
    print("Result:", result)
    return result  # Expected: 300

def test_branch_with_loop_vs_direct() -> int:
    """One branch uses loop, other uses direct append"""
    print("Test: branch loop vs direct")
    collection = []

    use_loop: int = 0

    if use_loop:
        i: int = 0
        while i < 5:
            collection.append(i * 10)
            i = i + 1
    else:
        collection.append(10)
        collection.append(20)
        collection.append(30)

    # collection: list[int]
    size: int = len(collection)
    print("Collection size:", size)
    return size  # Expected: 3

def test_deeply_nested_branches() -> int:
    """Deeply nested conditional branches"""
    print("Test: deeply nested branches")
    result = []

    a: int = 1
    b: int = 1
    c: int = 0

    if a:
        if b:
            if c:
                result.append(1)
            else:
                result.append(2)
        else:
            result.append(3)
    else:
        result.append(4)

    # result: list[int]
    value: int = result[0]
    print("Value:", value)
    return value  # Expected: 2

def test_conditional_with_multiple_containers() -> int:
    """Multiple containers affected by conditional"""
    print("Test: conditional multiple containers")
    list1 = []
    list2 = []

    mode: int = 1

    if mode == 1:
        list1.append(10)
        list2.append(20)
    else:
        list1.append(30)
        list2.append(40)

    # Both: list[int]
    result: int = list1[0] + list2[0]
    print("Sum:", result)
    return result  # Expected: 30

def test_switch_case_pattern() -> int:
    """Simulate switch/case with if/elif chain"""
    print("Test: switch case pattern")
    mapping = {}

    case: int = 3

    if case == 1:
        mapping[case] = 100
    elif case == 2:
        mapping[case] = 200
    elif case == 3:
        mapping[case] = 300
    elif case == 4:
        mapping[case] = 400
    else:
        mapping[0] = 0

    # mapping: dict[int, int]
    result: int = mapping[3]
    print("Case 3 value:", result)
    return result  # Expected: 300
