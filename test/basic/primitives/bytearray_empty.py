# Test empty bytearray constructor

def test_bytearray_empty_constructor() -> int:
    """Test bytearray() with no arguments"""
    ba: bytearray = bytearray()
    return len(ba)  # Should return 0

def test_bytearray_empty_then_append() -> int:
    """Test empty bytearray then append"""
    ba: bytearray = bytearray()
    ba.append(65)  # Append 'A'
    ba.append(66)  # Append 'B'
    return len(ba)  # Should return 2
