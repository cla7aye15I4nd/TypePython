# Complex type inference tests: Loop-based inference
# These tests verify type inference in various loop patterns

def test_build_list_in_loop() -> int:
    """Build list element-by-element in loop"""
    print("Test: build list in loop")
    numbers = []

    i: int = 0
    while i < 10:
        numbers.append(i * i)
        i = i + 1

    # numbers: list[int]
    # Access element: numbers[5] = 25
    result: int = numbers[5]
    print("Value at index 5:", result)
    return result  # Expected: 25

def test_nested_loop_matrix() -> int:
    """Build 2D matrix using nested loops"""
    print("Test: nested loop matrix")
    matrix = []

    i: int = 0
    while i < 3:
        row = []
        j: int = 0
        while j < 3:
            row.append(i * 3 + j)
            j = j + 1
        matrix.append(row)
        i = i + 1

    # matrix: list[list[int]]
    # Access: matrix[1][2] = 1*3 + 2 = 5
    result: int = matrix[1][2]
    print("Matrix[1][2]:", result)
    return result  # Expected: 5

def test_accumulate_from_iteration() -> int:
    """Accumulate values from iterating over list"""
    print("Test: accumulate from iteration")
    source = [10, 20, 30, 40, 50]
    doubled = []

    for value in source:
        doubled.append(value * 2)

    # doubled: list[int] (inferred from loop variable type)
    result: int = doubled[2]
    print("Doubled value:", result)
    return result  # Expected: 60

def test_filter_with_loop() -> int:
    """Filter values using loop"""
    print("Test: filter with loop")
    all_numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    evens = []

    for num in all_numbers:
        if num % 2 == 0:
            evens.append(num)

    # evens: list[int]
    size: int = len(evens)
    print("Even count:", size)
    return size  # Expected: 5

def test_loop_with_dict_building() -> int:
    """Build dict in loop"""
    print("Test: dict building in loop")
    squares = {}

    i: int = 1
    while i <= 5:
        squares[i] = i * i
        i = i + 1

    # squares: dict[int, int]
    result: int = squares[4]
    print("Square of 4:", result)
    return result  # Expected: 16

def test_loop_dict_iteration() -> int:
    """Iterate over dict and build new structure"""
    print("Test: dict iteration")
    prices = {1: 100, 2: 200, 3: 300}
    price_list = []

    for item_id in prices:
        price_list.append(prices[item_id])

    # price_list: list[int]
    total: int = 0
    for price in price_list:
        total = total + price

    print("Total:", total)
    return total  # Expected: 600

def test_loop_with_set_building() -> int:
    """Build set in loop"""
    print("Test: set building in loop")
    seen = set()

    values = [1, 2, 3, 2, 1, 4, 3, 5]
    for val in values:
        seen.add(val)

    # seen: set[int]
    size: int = len(seen)
    print("Unique count:", size)
    return size  # Expected: 5

def test_loop_with_conditional_append() -> int:
    """Conditional appends in loop"""
    print("Test: conditional append in loop")
    positives = []
    negatives = []

    numbers = [-5, 3, -2, 8, -1, 6, 0]
    for num in numbers:
        if num > 0:
            positives.append(num)
        elif num < 0:
            negatives.append(num)

    # Both: list[int]
    pos_count: int = len(positives)
    neg_count: int = len(negatives)

    print("Positives:", pos_count, "Negatives:", neg_count)
    return pos_count + neg_count  # Expected: 6

def test_nested_loop_with_accumulation() -> int:
    """Nested loops with complex accumulation"""
    print("Test: nested loop accumulation")
    result_matrix = []

    i: int = 0
    while i < 4:
        row = []
        j: int = 0
        while j < 4:
            # Create multiplication table
            row.append((i + 1) * (j + 1))
            j = j + 1
        result_matrix.append(row)
        i = i + 1

    # result_matrix: list[list[int]]
    # Access result_matrix[2][3] = 3 * 4 = 12
    result: int = result_matrix[2][3]
    print("Multiplication table [2][3]:", result)
    return result  # Expected: 12

def test_loop_breaking_and_continuing() -> int:
    """Loop with break and continue affecting accumulation"""
    print("Test: loop with break/continue")
    collected = []

    i: int = 0
    while i < 20:
        if i > 10:
            # Break after collecting 0-10
            break
        if i % 2 == 1:
            # Skip odd numbers
            i = i + 1
            continue
        collected.append(i)
        i = i + 1

    # collected: list[int] = [0, 2, 4, 6, 8, 10]
    size: int = len(collected)
    print("Collected count:", size)
    return size  # Expected: 6

def test_three_level_nested_loop() -> int:
    """Three level nested loops building 3D structure"""
    print("Test: three level nested loop")
    cube = []

    x: int = 0
    while x < 2:
        plane = []
        y: int = 0
        while y < 2:
            line = []
            z: int = 0
            while z < 2:
                line.append(x * 4 + y * 2 + z)
                z = z + 1
            plane.append(line)
            y = y + 1
        cube.append(plane)
        x = x + 1

    # cube: list[list[list[int]]]
    # Access cube[1][1][1] = 1*4 + 1*2 + 1 = 7
    result: int = cube[1][1][1]
    print("Cube[1][1][1]:", result)
    return result  # Expected: 7

def test_loop_over_range_building_dict() -> int:
    """Loop over range building dict"""
    print("Test: loop over range with dict")
    # Simulate range(5) with a list
    range_values = [0, 1, 2, 3, 4]
    mapping = {}

    for i in range_values:
        mapping[i] = i * 100

    # mapping: dict[int, int]
    result: int = mapping[3]
    print("Mapping[3]:", result)
    return result  # Expected: 300

def test_enumerate_pattern() -> int:
    """Simulate enumerate pattern with index tracking"""
    print("Test: enumerate pattern")
    values = [10, 20, 30, 40, 50]
    indexed = {}

    idx: int = 0
    for val in values:
        indexed[idx] = val
        idx = idx + 1

    # indexed: dict[int, int]
    result: int = indexed[2]
    print("Indexed[2]:", result)
    return result  # Expected: 30
