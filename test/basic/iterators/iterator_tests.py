# Iterator tests - for loops with range and list

def test_for_range_one_arg() -> int:
    """Test for loop with range(stop)"""
    total: int = 0
    for i in range(5):
        total += i
    # 0+1+2+3+4 = 10
    if total == 10:
        return 1
    return 0

def test_for_range_two_args() -> int:
    """Test for loop with range(start, stop)"""
    total: int = 0
    for i in range(2, 5):
        total += i
    # 2+3+4 = 9
    if total == 9:
        return 1
    return 0

def test_for_range_three_args() -> int:
    """Test for loop with range(start, stop, step)"""
    total: int = 0
    for i in range(0, 10, 2):
        total += i
    # 0+2+4+6+8 = 20
    if total == 20:
        return 1
    return 0

def test_for_list_basic() -> int:
    """Test for loop iterating over list"""
    numbers: list[int] = [1, 2, 3, 4, 5]
    total: int = 0
    for n in numbers:
        total += n
    # 1+2+3+4+5 = 15
    if total == 15:
        return 1
    return 0

def test_for_list_modify() -> int:
    """Test modifying values during list iteration"""
    values: list[int] = [10, 20, 30]
    doubled: int = 0
    for v in values:
        doubled += v * 2
    # 20+40+60 = 120
    if doubled == 120:
        return 1
    return 0

def test_for_nested_range() -> int:
    """Test nested for loops with range"""
    count: int = 0
    for i in range(3):
        for j in range(4):
            count += 1
    # 3 * 4 = 12
    if count == 12:
        return 1
    return 0

def test_for_nested_list() -> int:
    """Test nested for loops with lists"""
    outer: list[int] = [1, 2]
    inner: list[int] = [10, 20, 30]
    sum_all: int = 0
    for a in outer:
        for b in inner:
            sum_all += a * b
    # 1*(10+20+30) + 2*(10+20+30) = 60 + 120 = 180
    if sum_all == 180:
        return 1
    return 0

def test_for_nested_mixed() -> int:
    """Test nested for loops mixing range and list"""
    items: list[int] = [1, 2, 3]
    total: int = 0
    for i in range(2):
        for x in items:
            total += x * (i + 1)
    # i=0: 1+2+3=6, i=1: 2+4+6=12, total=18
    if total == 18:
        return 1
    return 0

def test_for_triple_nested() -> int:
    """Test triple nested for loops"""
    count: int = 0
    for i in range(2):
        for j in range(3):
            for k in range(4):
                count += 1
    # 2 * 3 * 4 = 24
    if count == 24:
        return 1
    return 0
