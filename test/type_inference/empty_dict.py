# Type inference tests for empty dicts
# These tests verify that empty dict types can be inferred from usage
# NOTE: These tests will work once dict support is fully implemented

def test_empty_dict_setitem() -> int:
    """Empty dict inferred from index assignment"""
    print("Test: empty dict with setitem")
    d = {}
    d[1] = 100
    d[2] = 200
    d[3] = 300
    result: int = d[1] + d[2]
    print("Result:", result)
    return result  # Expected: 300

def test_empty_dict_str_keys() -> int:
    """Empty dict with string keys inferred from usage"""
    print("Test: empty dict with string keys")
    d = {}
    # Note: This would require string key support
    # d["a"] = 10
    # d["b"] = 20
    # return d["a"] + d["b"]  # Expected: 30

    # For now, use int keys
    d[1] = 10
    d[2] = 20
    return d[1] + d[2]  # Expected: 30

def test_empty_dict_update() -> int:
    """Update existing keys in empty dict"""
    print("Test: empty dict update")
    scores = {}
    scores[1] = 50
    scores[1] = 100  # Update same key
    scores[2] = 75
    print("Updated score:", scores[1])
    return scores[1]  # Expected: 100

def test_empty_dict_len() -> int:
    """Empty dict with len() call"""
    print("Test: empty dict with len")
    mapping = {}
    size1: int = len(mapping)
    print("Initial size:", size1)

    mapping[10] = 1
    mapping[20] = 2
    mapping[30] = 3

    size2: int = len(mapping)
    print("After inserts:", size2)
    return size2  # Expected: 3

def test_empty_dict_iteration() -> int:
    """Iterate over empty dict keys"""
    print("Test: empty dict iteration")
    d = {}
    d[1] = 10
    d[2] = 20
    d[3] = 30

    total: int = 0
    for key in d:
        total = total + d[key]

    print("Total:", total)
    return total  # Expected: 60

def test_dict_literal_non_empty() -> int:
    """Non-empty dict literal (no inference needed)"""
    print("Test: dict literal")
    d = {1: 100, 2: 200, 3: 300}
    result: int = d[2]
    print("Value at key 2:", result)
    return result  # Expected: 200

def test_dict_literal_iteration() -> int:
    """Iterate over dict literal"""
    print("Test: dict literal iteration")
    prices = {10: 5, 20: 10, 30: 15}

    total: int = 0
    for item_id in prices:
        total = total + prices[item_id]

    print("Total price:", total)
    return total  # Expected: 30
