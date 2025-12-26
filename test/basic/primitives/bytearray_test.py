# Test bytearray type functionality

def test_bytearray_create() -> int:
    ba: bytearray = bytearray(b"hello")
    return ba[0]  # Should return 104 (ASCII 'h')

def test_bytearray_len() -> int:
    ba: bytearray = bytearray(b"world")
    return len(ba)  # Should return 5

def test_bytearray_empty() -> int:
    ba: bytearray = bytearray(b"")
    return len(ba)  # Should return 0

def test_bytearray_append() -> int:
    ba: bytearray = bytearray(b"ab")
    ba.append(99)  # Append 'c'
    return len(ba)  # Should return 3

def test_bytearray_append_value() -> int:
    ba: bytearray = bytearray(b"ab")
    ba.append(99)
    return ba[2]  # Should return 99 (ASCII 'c')

def test_bytearray_set() -> int:
    ba: bytearray = bytearray(b"abc")
    ba[0] = 65  # Change 'a' to 'A'
    return ba[0]  # Should return 65 (ASCII 'A')
