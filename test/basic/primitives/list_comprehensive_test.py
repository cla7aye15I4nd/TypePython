# Comprehensive tests for list class
# Tests: __init__, __len__, __getitem__, __setitem__, __str__, __repr__, append

# ============ __init__ and __len__ tests ============

def test_list_init_single_then_check() -> int:
    """Test single element list length"""
    nums: list[int] = [1]
    return len(nums)  # Expected: 1

def test_list_init_single() -> int:
    """Test single element list"""
    nums: list[int] = [42]
    return nums[0]  # Expected: 42

def test_list_init_multiple() -> int:
    """Test multiple element list"""
    nums: list[int] = [1, 2, 3, 4, 5]
    return len(nums)  # Expected: 5

def test_list_init_negative() -> int:
    """Test list with negative values"""
    nums: list[int] = [-1, -2, -3]
    return nums[0] + nums[1] + nums[2]  # Expected: -6

def test_list_init_zeros() -> int:
    """Test list with all zeros"""
    nums: list[int] = [0, 0, 0]
    return len(nums)  # Expected: 3

def test_list_init_large_values() -> int:
    """Test list with large values"""
    nums: list[int] = [1000000, 2000000]
    return nums[0]  # Expected: 1000000

def test_list_len_after_append() -> int:
    """Test len() after appending"""
    nums: list[int] = [1, 2]
    nums.append(3)
    return len(nums)  # Expected: 3

# ============ __getitem__ tests ============

def test_list_getitem_first() -> int:
    """Test getting first element"""
    nums: list[int] = [100, 200, 300]
    return nums[0]  # Expected: 100

def test_list_getitem_last() -> int:
    """Test getting last element"""
    nums: list[int] = [10, 20, 30, 40, 50]
    return nums[4]  # Expected: 50

def test_list_getitem_middle() -> int:
    """Test getting middle element"""
    nums: list[int] = [1, 2, 3, 4, 5]
    return nums[2]  # Expected: 3

def test_list_getitem_negative_value() -> int:
    """Test getting negative value"""
    nums: list[int] = [-10, -20, -30]
    return nums[1]  # Expected: -20

def test_list_getitem_after_append() -> int:
    """Test getting appended element"""
    nums: list[int] = [1, 2]
    nums.append(999)
    return nums[2]  # Expected: 999

def test_list_getitem_after_setitem() -> int:
    """Test getting element after modification"""
    nums: list[int] = [1, 2, 3]
    nums[1] = 200
    return nums[1]  # Expected: 200

# ============ __setitem__ tests ============

def test_list_setitem_first() -> int:
    """Test setting first element"""
    nums: list[int] = [1, 2, 3]
    nums[0] = 100
    return nums[0]  # Expected: 100

def test_list_setitem_last() -> int:
    """Test setting last element"""
    nums: list[int] = [1, 2, 3]
    nums[2] = 300
    return nums[2]  # Expected: 300

def test_list_setitem_middle() -> int:
    """Test setting middle element"""
    nums: list[int] = [10, 20, 30, 40, 50]
    nums[2] = 999
    return nums[2]  # Expected: 999

def test_list_setitem_to_zero() -> int:
    """Test setting element to zero"""
    nums: list[int] = [1, 2, 3]
    nums[1] = 0
    return nums[1]  # Expected: 0

def test_list_setitem_to_negative() -> int:
    """Test setting element to negative"""
    nums: list[int] = [1, 2, 3]
    nums[0] = -100
    return nums[0]  # Expected: -100

def test_list_setitem_preserve_others() -> int:
    """Test that setitem preserves other elements"""
    nums: list[int] = [1, 2, 3]
    nums[1] = 999
    return nums[0] + nums[2]  # Expected: 4 (1+3, unchanged)

# ============ append tests ============

def test_list_append_to_one() -> int:
    """Test appending to single element list"""
    nums: list[int] = [1]
    nums.append(42)
    return nums[1]  # Expected: 42

def test_list_append_single() -> int:
    """Test appending single value"""
    nums: list[int] = [1, 2]
    nums.append(3)
    return nums[2]  # Expected: 3

def test_list_append_multiple() -> int:
    """Test appending multiple values"""
    nums: list[int] = [1]
    nums.append(10)
    nums.append(20)
    nums.append(30)
    return nums[1] + nums[2] + nums[3]  # Expected: 60

def test_list_append_negative() -> int:
    """Test appending negative value"""
    nums: list[int] = [1]
    nums.append(-5)
    return nums[1]  # Expected: -5

def test_list_append_zero() -> int:
    """Test appending zero"""
    nums: list[int] = [1, 2]
    nums.append(0)
    return nums[2]  # Expected: 0

def test_list_append_grow_capacity() -> int:
    """Test appending beyond initial capacity (8)"""
    nums: list[int] = [0]
    nums.append(1)
    nums.append(2)
    nums.append(3)
    nums.append(4)
    nums.append(5)
    nums.append(6)
    nums.append(7)
    nums.append(8)
    nums.append(9)  # Should trigger capacity growth
    nums.append(10)
    return len(nums)  # Expected: 11

def test_list_append_grow_and_access() -> int:
    """Test accessing elements after capacity growth"""
    nums: list[int] = [0]
    i: int = 1
    while i < 20:
        nums.append(i * 10)
        i = i + 1
    return nums[15]  # Expected: 150

# ============ Mutation operation sequences ============

def test_list_append_then_set() -> int:
    """Test append followed by setitem"""
    nums: list[int] = [1, 2]
    nums.append(3)
    nums[2] = 300
    return nums[2]  # Expected: 300

def test_list_set_then_append() -> int:
    """Test setitem followed by append"""
    nums: list[int] = [1, 2, 3]
    nums[1] = 200
    nums.append(4)
    return nums[1] + nums[3]  # Expected: 200+4 = 204

def test_list_multiple_mutations() -> int:
    """Test sequence of mutations"""
    nums: list[int] = [1, 2, 3]
    nums[0] = 10
    nums.append(4)
    nums[1] = 20
    nums.append(5)
    nums[2] = 30
    return nums[0] + nums[1] + nums[2] + nums[3] + nums[4]  # Expected: 10+20+30+4+5 = 69

# ============ Print tests ============

def test_list_print_one() -> int:
    """Test printing single element list"""
    nums: list[int] = [1]
    print(nums)  # Expected output: [1]
    return 1

def test_list_print_single() -> int:
    """Test printing single element list"""
    nums: list[int] = [42]
    print(nums)  # Expected output: [42]
    return 1

def test_list_print_multiple() -> int:
    """Test printing multiple element list"""
    nums: list[int] = [1, 2, 3]
    print(nums)  # Expected output: [1, 2, 3]
    return 1

def test_list_print_negative() -> int:
    """Test printing list with negative values"""
    nums: list[int] = [-1, 0, 1]
    print(nums)  # Expected output: [-1, 0, 1]
    return 1

def test_list_print_after_append() -> int:
    """Test printing list after append"""
    nums: list[int] = [1, 2]
    nums.append(3)
    print(nums)  # Expected output: [1, 2, 3]
    return 1

def test_list_print_after_setitem() -> int:
    """Test printing list after setitem"""
    nums: list[int] = [1, 2, 3]
    nums[1] = 999
    print(nums)  # Expected output: [1, 999, 3]
    return 1

def test_list_print_large_values() -> int:
    """Test printing list with large values"""
    nums: list[int] = [1000000, 2000000, 3000000]
    print(nums)  # Expected output: [1000000, 2000000, 3000000]
    return 1

# ============ Edge cases ============

def test_list_sum_after_mutations() -> int:
    """Test sum of elements after mutations"""
    nums: list[int] = [1, 2, 3, 4, 5]
    nums[0] = 10
    nums[1] = 20
    nums[2] = 30
    return nums[0] + nums[1] + nums[2] + nums[3] + nums[4]  # Expected: 10+20+30+4+5 = 69

def test_list_all_same_value() -> int:
    """Test list with all same values"""
    nums: list[int] = [7, 7, 7, 7, 7]
    return nums[0] + nums[2] + nums[4]  # Expected: 21

def test_list_alternating_values() -> int:
    """Test list with alternating values"""
    nums: list[int] = [1, 0, 1, 0, 1]
    return nums[0] + nums[1] + nums[2] + nums[3] + nums[4]  # Expected: 3

def test_list_single_zero() -> int:
    """Test single zero list"""
    nums: list[int] = [0]
    return nums[0]  # Expected: 0

def test_list_len_unchanged_after_setitem() -> int:
    """Test that setitem doesn't change length"""
    nums: list[int] = [1, 2, 3, 4, 5]
    nums[2] = 999
    return len(nums)  # Expected: 5

def main() -> int:
    failed: int = 0

    # __init__ and __len__ tests
    if test_list_init_single_then_check() != 1:
        print(1)
        failed = failed + 1
    if test_list_init_single() != 42:
        print(2)
        failed = failed + 1
    if test_list_init_multiple() != 5:
        print(3)
        failed = failed + 1
    if test_list_init_negative() != -6:
        print(4)
        failed = failed + 1
    if test_list_init_zeros() != 3:
        print(5)
        failed = failed + 1
    if test_list_init_large_values() != 1000000:
        print(6)
        failed = failed + 1
    if test_list_len_after_append() != 3:
        print(7)
        failed = failed + 1

    # __getitem__ tests
    if test_list_getitem_first() != 100:
        print(8)
        failed = failed + 1
    if test_list_getitem_last() != 50:
        print(9)
        failed = failed + 1
    if test_list_getitem_middle() != 3:
        print(10)
        failed = failed + 1
    if test_list_getitem_negative_value() != -20:
        print(11)
        failed = failed + 1
    if test_list_getitem_after_append() != 999:
        print(12)
        failed = failed + 1
    if test_list_getitem_after_setitem() != 200:
        print(13)
        failed = failed + 1

    # __setitem__ tests
    if test_list_setitem_first() != 100:
        print(14)
        failed = failed + 1
    if test_list_setitem_last() != 300:
        print(15)
        failed = failed + 1
    if test_list_setitem_middle() != 999:
        print(16)
        failed = failed + 1
    if test_list_setitem_to_zero() != 0:
        print(17)
        failed = failed + 1
    if test_list_setitem_to_negative() != -100:
        print(18)
        failed = failed + 1
    if test_list_setitem_preserve_others() != 4:
        print(19)
        failed = failed + 1

    # append tests
    if test_list_append_to_one() != 42:
        print(20)
        failed = failed + 1
    if test_list_append_single() != 3:
        print(21)
        failed = failed + 1
    if test_list_append_multiple() != 60:
        print(22)
        failed = failed + 1
    if test_list_append_negative() != -5:
        print(23)
        failed = failed + 1
    if test_list_append_zero() != 0:
        print(24)
        failed = failed + 1
    if test_list_append_grow_capacity() != 11:
        print(25)
        failed = failed + 1
    if test_list_append_grow_and_access() != 150:
        print(26)
        failed = failed + 1

    # Mutation operation sequences
    if test_list_append_then_set() != 300:
        print(27)
        failed = failed + 1
    if test_list_set_then_append() != 204:
        print(28)
        failed = failed + 1
    if test_list_multiple_mutations() != 69:
        print(29)
        failed = failed + 1

    # Print tests
    if test_list_print_one() != 1:
        print(30)
        failed = failed + 1
    if test_list_print_single() != 1:
        print(31)
        failed = failed + 1
    if test_list_print_multiple() != 1:
        print(32)
        failed = failed + 1
    if test_list_print_negative() != 1:
        print(33)
        failed = failed + 1
    if test_list_print_after_append() != 1:
        print(34)
        failed = failed + 1
    if test_list_print_after_setitem() != 1:
        print(35)
        failed = failed + 1
    if test_list_print_large_values() != 1:
        print(36)
        failed = failed + 1

    # Edge cases
    if test_list_sum_after_mutations() != 69:
        print(37)
        failed = failed + 1
    if test_list_all_same_value() != 21:
        print(38)
        failed = failed + 1
    if test_list_alternating_values() != 3:
        print(39)
        failed = failed + 1
    if test_list_single_zero() != 0:
        print(40)
        failed = failed + 1
    if test_list_len_unchanged_after_setitem() != 5:
        print(41)
        failed = failed + 1

    if failed == 0:
        print(0)
    return failed

main()
