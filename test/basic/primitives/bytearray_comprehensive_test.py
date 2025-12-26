# Comprehensive tests for bytearray class
# Tests: __init__, __len__, __getitem__, __setitem__, __str__, __repr__, append

# ============ __init__ and __len__ tests ============

def test_bytearray_init_from_bytes() -> int:
    """Test bytearray initialization from bytes literal"""
    ba: bytearray = bytearray(b"hello")
    return len(ba)  # Expected: 5

def test_bytearray_init_empty() -> int:
    """Test empty bytearray initialization"""
    ba: bytearray = bytearray(b"")
    return len(ba)  # Expected: 0

def test_bytearray_init_single() -> int:
    """Test single byte bytearray"""
    ba: bytearray = bytearray(b"x")
    return ba[0]  # Expected: 120 (ASCII 'x')

def test_bytearray_len_after_append() -> int:
    """Test len() after appending"""
    ba: bytearray = bytearray(b"ab")
    ba.append(99)
    return len(ba)  # Expected: 3

def test_bytearray_len_multiple_appends() -> int:
    """Test len() after multiple appends"""
    ba: bytearray = bytearray(b"")
    ba.append(1)
    ba.append(2)
    ba.append(3)
    return len(ba)  # Expected: 3

# ============ __getitem__ tests ============

def test_bytearray_getitem_first() -> int:
    """Test getting first element"""
    ba: bytearray = bytearray(b"hello")
    return ba[0]  # Expected: 104 (ASCII 'h')

def test_bytearray_getitem_last() -> int:
    """Test getting last element"""
    ba: bytearray = bytearray(b"hello")
    return ba[4]  # Expected: 111 (ASCII 'o')

def test_bytearray_getitem_after_append() -> int:
    """Test getting appended element"""
    ba: bytearray = bytearray(b"ab")
    ba.append(99)
    return ba[2]  # Expected: 99 (ASCII 'c')

def test_bytearray_getitem_after_setitem() -> int:
    """Test getting element after modification"""
    ba: bytearray = bytearray(b"abc")
    ba[1] = 88
    return ba[1]  # Expected: 88 (ASCII 'X')

# ============ __setitem__ tests ============

def test_bytearray_setitem_first() -> int:
    """Test setting first element"""
    ba: bytearray = bytearray(b"abc")
    ba[0] = 65
    return ba[0]  # Expected: 65 (ASCII 'A')

def test_bytearray_setitem_last() -> int:
    """Test setting last element"""
    ba: bytearray = bytearray(b"abc")
    ba[2] = 90
    return ba[2]  # Expected: 90 (ASCII 'Z')

def test_bytearray_setitem_middle() -> int:
    """Test setting middle element"""
    ba: bytearray = bytearray(b"hello")
    ba[2] = 88
    return ba[2]  # Expected: 88 (ASCII 'X')

def test_bytearray_setitem_to_zero() -> int:
    """Test setting element to zero"""
    ba: bytearray = bytearray(b"abc")
    ba[0] = 0
    return ba[0]  # Expected: 0

def test_bytearray_setitem_to_max() -> int:
    """Test setting element to 255"""
    ba: bytearray = bytearray(b"abc")
    ba[0] = 255
    return ba[0]  # Expected: 255

def test_bytearray_setitem_preserve_others() -> int:
    """Test that setitem preserves other elements"""
    ba: bytearray = bytearray(b"abc")
    ba[1] = 88
    return ba[0] + ba[2]  # Expected: 97+99 = 196 (a+c unchanged)

# ============ append tests ============

def test_bytearray_append_single() -> int:
    """Test appending single value"""
    ba: bytearray = bytearray(b"")
    ba.append(65)
    return ba[0]  # Expected: 65 (ASCII 'A')

def test_bytearray_append_multiple() -> int:
    """Test appending multiple values"""
    ba: bytearray = bytearray(b"")
    ba.append(1)
    ba.append(2)
    ba.append(3)
    return ba[0] + ba[1] + ba[2]  # Expected: 6

def test_bytearray_append_zero() -> int:
    """Test appending zero"""
    ba: bytearray = bytearray(b"a")
    ba.append(0)
    return ba[1]  # Expected: 0

def test_bytearray_append_max() -> int:
    """Test appending 255"""
    ba: bytearray = bytearray(b"")
    ba.append(255)
    return ba[0]  # Expected: 255

def test_bytearray_append_grow_capacity() -> int:
    """Test appending beyond initial capacity (8)"""
    ba: bytearray = bytearray(b"")
    ba.append(1)
    ba.append(2)
    ba.append(3)
    ba.append(4)
    ba.append(5)
    ba.append(6)
    ba.append(7)
    ba.append(8)
    ba.append(9)  # Should trigger capacity growth
    ba.append(10)
    return len(ba)  # Expected: 10

# ============ Mutation operation sequences ============

def test_bytearray_append_then_set() -> int:
    """Test append followed by setitem"""
    ba: bytearray = bytearray(b"ab")
    ba.append(99)
    ba[2] = 100
    return ba[2]  # Expected: 100 (ASCII 'd')

def test_bytearray_set_then_append() -> int:
    """Test setitem followed by append"""
    ba: bytearray = bytearray(b"abc")
    ba[1] = 66
    ba.append(68)
    return ba[1] + ba[3]  # Expected: 66+68 = 134

def test_bytearray_multiple_mutations() -> int:
    """Test sequence of mutations"""
    ba: bytearray = bytearray(b"abc")
    ba[0] = 1
    ba.append(4)
    ba[1] = 2
    ba.append(5)
    ba[2] = 3
    return ba[0] + ba[1] + ba[2] + ba[3] + ba[4]  # Expected: 1+2+3+4+5 = 15

# ============ Print tests ============

def test_bytearray_print_simple() -> int:
    """Test printing simple bytearray"""
    ba: bytearray = bytearray(b"hello")
    print(ba)  # Expected output: bytearray(b'hello')
    return 1

def test_bytearray_print_empty() -> int:
    """Test printing empty bytearray"""
    ba: bytearray = bytearray(b"")
    print(ba)  # Expected output: bytearray(b'')
    return 1

def test_bytearray_print_after_append() -> int:
    """Test printing bytearray after append"""
    ba: bytearray = bytearray(b"ab")
    ba.append(99)
    print(ba)  # Expected output: bytearray(b'abc')
    return 1

# ============ Edge cases ============

def test_bytearray_sum_after_mutations() -> int:
    """Test sum of elements after mutations"""
    ba: bytearray = bytearray(b"abc")
    ba[0] = 10
    ba[1] = 20
    ba[2] = 30
    return ba[0] + ba[1] + ba[2]  # Expected: 60

def test_bytearray_len_unchanged_after_setitem() -> int:
    """Test that setitem doesn't change length"""
    ba: bytearray = bytearray(b"hello")
    ba[0] = 72
    ba[4] = 79
    return len(ba)  # Expected: 5

def main() -> int:
    failed: int = 0

    # __init__ and __len__ tests
    if test_bytearray_init_from_bytes() != 5:
        print(1)
        failed = failed + 1
    if test_bytearray_init_empty() != 0:
        print(2)
        failed = failed + 1
    if test_bytearray_init_single() != 120:
        print(3)
        failed = failed + 1
    if test_bytearray_len_after_append() != 3:
        print(4)
        failed = failed + 1
    if test_bytearray_len_multiple_appends() != 3:
        print(5)
        failed = failed + 1

    # __getitem__ tests
    if test_bytearray_getitem_first() != 104:
        print(6)
        failed = failed + 1
    if test_bytearray_getitem_last() != 111:
        print(7)
        failed = failed + 1
    if test_bytearray_getitem_after_append() != 99:
        print(8)
        failed = failed + 1
    if test_bytearray_getitem_after_setitem() != 88:
        print(9)
        failed = failed + 1

    # __setitem__ tests
    if test_bytearray_setitem_first() != 65:
        print(10)
        failed = failed + 1
    if test_bytearray_setitem_last() != 90:
        print(11)
        failed = failed + 1
    if test_bytearray_setitem_middle() != 88:
        print(12)
        failed = failed + 1
    if test_bytearray_setitem_to_zero() != 0:
        print(13)
        failed = failed + 1
    if test_bytearray_setitem_to_max() != 255:
        print(14)
        failed = failed + 1
    if test_bytearray_setitem_preserve_others() != 196:
        print(15)
        failed = failed + 1

    # append tests
    if test_bytearray_append_single() != 65:
        print(16)
        failed = failed + 1
    if test_bytearray_append_multiple() != 6:
        print(17)
        failed = failed + 1
    if test_bytearray_append_zero() != 0:
        print(18)
        failed = failed + 1
    if test_bytearray_append_max() != 255:
        print(19)
        failed = failed + 1
    if test_bytearray_append_grow_capacity() != 10:
        print(20)
        failed = failed + 1

    # Mutation operation sequences
    if test_bytearray_append_then_set() != 100:
        print(21)
        failed = failed + 1
    if test_bytearray_set_then_append() != 134:
        print(22)
        failed = failed + 1
    if test_bytearray_multiple_mutations() != 15:
        print(23)
        failed = failed + 1

    # Print tests (output verification)
    if test_bytearray_print_simple() != 1:
        print(24)
        failed = failed + 1
    if test_bytearray_print_empty() != 1:
        print(25)
        failed = failed + 1
    if test_bytearray_print_after_append() != 1:
        print(26)
        failed = failed + 1

    # Edge cases
    if test_bytearray_sum_after_mutations() != 60:
        print(27)
        failed = failed + 1
    if test_bytearray_len_unchanged_after_setitem() != 5:
        print(28)
        failed = failed + 1

    if failed == 0:
        print(0)
    return failed

main()
