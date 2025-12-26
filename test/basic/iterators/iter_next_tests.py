# Tests for iter() and next() builtin functions

def test_iter_next_basic() -> int:
    """Test basic iter() and next() on a list"""
    nums: list[int] = [10, 20, 30]
    it = iter(nums)
    first: int = next(it)
    second: int = next(it)
    # 10 + 20 = 30
    return first + second

def test_iter_next_all_elements() -> int:
    """Test iter/next to consume all elements"""
    vals: list[int] = [1, 2, 3, 4, 5]
    it = iter(vals)
    total: int = 0
    total = total + next(it)
    total = total + next(it)
    total = total + next(it)
    total = total + next(it)
    total = total + next(it)
    # 1+2+3+4+5 = 15
    return total

def test_iter_range() -> int:
    """Test iter() on range object"""
    r = range(5)
    it = iter(r)
    first: int = next(it)
    second: int = next(it)
    # 0 + 1 = 1
    return first + second
