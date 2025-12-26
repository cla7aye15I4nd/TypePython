# Complex type inference tests: Cross-variable type flow
# These tests verify type inference when types flow between variables

def test_variable_to_variable_list() -> int:
    """Type flows from one list to another via append"""
    print("Test: variable to variable - list")
    source = []
    target = []

    source.append(10)
    source.append(20)

    # Type flows: target gets type from source
    target.append(source)

    # target is now list[list[int]]
    result: int = target[0][0] + target[0][1]
    print("Result:", result)
    return result  # Expected: 30

def test_chain_of_assignments() -> int:
    """Type flows through chain of assignments"""
    print("Test: chain of assignments")
    a = []
    b = []
    c = []

    c.append(100)  # c: list[int]
    b.append(c)    # b: list[list[int]]
    a.append(b)    # a: list[list[list[int]]]

    result: int = a[0][0][0]
    print("Chained value:", result)
    return result  # Expected: 100

def test_multiple_sources_same_target() -> int:
    """Multiple variables contribute to type of target"""
    print("Test: multiple sources")
    target = []

    source1 = []
    source1.append(10)

    source2 = []
    source2.append(20)

    # Both contribute to target's element type
    target.append(source1)
    target.append(source2)

    # target: list[list[int]]
    result: int = target[0][0] + target[1][0]
    print("Result:", result)
    return result  # Expected: 30

def test_dict_value_propagation() -> int:
    """Type propagates through dict values"""
    print("Test: dict value propagation")
    container = {}
    data = []

    data.append(42)
    data.append(84)

    container[1] = data

    # container: dict[int, list[int]]
    result: int = container[1][0] + container[1][1]
    print("Result:", result)
    return result  # Expected: 126

def test_bidirectional_flow() -> int:
    """Type flows in both directions"""
    print("Test: bidirectional flow")
    x = []
    y = []

    # y gets int from direct append
    y.append(5)

    # x gets list[int] from y
    x.append(y)

    # Now append more to y (type already known)
    y.append(10)

    result: int = x[0][0] + x[0][1]
    print("Result:", result)
    return result  # Expected: 15

def test_loop_accumulation() -> int:
    """Type inference across loop iterations"""
    print("Test: loop accumulation")
    results = []

    i: int = 0
    while i < 5:
        item = []
        item.append(i)
        item.append(i * 10)

        results.append(item)
        i = i + 1

    # results: list[list[int]]
    # Access results[2][1] should be 20
    result: int = results[2][1]
    print("Loop result:", result)
    return result  # Expected: 20

def test_conditional_flow() -> int:
    """Type flows through conditional branches"""
    print("Test: conditional flow")
    container = []
    branch1 = []
    branch2 = []

    condition: int = 1

    if condition:
        branch1.append(100)
        container.append(branch1)
    else:
        branch2.append(200)
        container.append(branch2)

    # container: list[list[int]] in both branches
    result: int = container[0][0]
    print("Conditional result:", result)
    return result  # Expected: 100

def test_function_local_flow() -> int:
    """Type flows within function scope"""
    print("Test: function local flow")

    # Simulate inner function by using separate scope
    accumulator = []
    temp_list = []

    # Build temp_list
    temp_list.append(1)
    temp_list.append(2)
    temp_list.append(3)

    # Add to accumulator
    accumulator.append(temp_list)

    # Create another temp
    temp_list2 = []
    temp_list2.append(4)
    temp_list2.append(5)
    accumulator.append(temp_list2)

    # Sum all nested values
    total: int = 0
    i: int = 0
    while i < 2:
        j: int = 0
        inner_len: int = len(accumulator[i])
        while j < inner_len:
            total = total + accumulator[i][j]
            j = j + 1
        i = i + 1

    print("Total:", total)
    return total  # Expected: 15

def test_set_to_list_flow() -> int:
    """Type flows from set to list via iteration"""
    print("Test: set to list flow")
    unique_values = set()
    unique_values.add(10)
    unique_values.add(20)
    unique_values.add(30)

    # Convert set to list
    converted = []
    for value in unique_values:
        converted.append(value)

    # converted: list[int] (element type from set)
    size: int = len(converted)
    print("Converted size:", size)
    return size  # Expected: 3

def test_dict_to_list_keys() -> int:
    """Type flows from dict keys to list"""
    print("Test: dict keys to list")
    mapping = {}
    mapping[10] = 100
    mapping[20] = 200
    mapping[30] = 300

    keys_list = []
    for key in mapping:
        keys_list.append(key)

    # keys_list: list[int]
    size: int = len(keys_list)
    print("Keys count:", size)
    return size  # Expected: 3
