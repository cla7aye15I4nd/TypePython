# Type inference tests for empty lists
# These tests verify that empty list types can be inferred from usage

def test_empty_list_append() -> int:
    """Empty list inferred from append call"""
    print("Test: empty list with append")
    x = []
    x.append(5)
    x.append(10)
    x.append(15)
    result: int = x[0] + x[1] + x[2]
    print("Result:", result)
    return result  # Expected: 30

def test_empty_list_setitem() -> int:
    """Empty list inferred from index assignment"""
    print("Test: empty list with setitem")
    nums = []
    nums.append(0)  # Initialize with at least one element
    nums[0] = 100
    print("Value:", nums[0])
    return nums[0]  # Expected: 100

def test_empty_list_in_loop() -> int:
    """Empty list used in a loop"""
    print("Test: empty list in loop")
    items = []
    i: int = 0
    while i < 5:
        items.append(i * 10)
        i = i + 1

    total: int = 0
    j: int = 0
    while j < 5:
        total = total + items[j]
        j = j + 1

    print("Total:", total)
    return total  # Expected: 0 + 10 + 20 + 30 + 40 = 100

def test_empty_list_len() -> int:
    """Empty list with len() call"""
    print("Test: empty list with len")
    data = []
    size1: int = len(data)
    print("Initial size:", size1)

    data.append(42)
    size2: int = len(data)
    print("After append:", size2)

    return size2  # Expected: 1

def test_empty_list_multiple_appends() -> int:
    """Multiple appends with different expressions"""
    print("Test: multiple appends")
    values = []
    values.append(10)
    values.append(20)
    values.append(30)

    sum: int = 0
    i: int = 0
    while i < len(values):
        sum = sum + values[i]
        i = i + 1

    print("Sum:", sum)
    return sum  # Expected: 60

def test_nested_list_access() -> int:
    """Empty list with nested operations"""
    print("Test: nested list access")
    collection = []
    collection.append(1)
    collection.append(2)
    collection.append(3)

    # Access and multiply
    result: int = collection[0] * collection[1] * collection[2]
    print("Product:", result)
    return result  # Expected: 6
