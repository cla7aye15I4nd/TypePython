# Comprehensive tests for bytes class
# Tests: __len__, __getitem__, __str__, __repr__

# ============ __len__ tests ============

def test_bytes_len_simple() -> int:
    """Test len() on simple bytes"""
    b: bytes = b"hello"
    return len(b)  # Expected: 5

def test_bytes_len_empty() -> int:
    """Test len() on empty bytes"""
    b: bytes = b""
    return len(b)  # Expected: 0

def test_bytes_len_single() -> int:
    """Test len() on single byte"""
    b: bytes = b"x"
    return len(b)  # Expected: 1

# ============ __getitem__ tests ============

def test_bytes_getitem_first() -> int:
    """Test getting first byte"""
    b: bytes = b"hello"
    return b[0]  # Expected: 104 (ASCII 'h')

def test_bytes_getitem_last() -> int:
    """Test getting last byte"""
    b: bytes = b"hello"
    return b[4]  # Expected: 111 (ASCII 'o')

def test_bytes_getitem_middle() -> int:
    """Test getting middle byte"""
    b: bytes = b"hello"
    return b[2]  # Expected: 108 (ASCII 'l')

def test_bytes_getitem_digit() -> int:
    """Test getting ASCII digit byte"""
    b: bytes = b"0123"
    return b[0]  # Expected: 48 (ASCII '0')

def test_bytes_getitem_space() -> int:
    """Test getting space byte"""
    b: bytes = b" x"
    return b[0]  # Expected: 32 (ASCII space)

# ============ Non-printable bytes ============

def test_bytes_null_byte() -> int:
    """Test bytes with null byte"""
    b: bytes = b"\x00"
    return b[0]  # Expected: 0

def test_bytes_high_value() -> int:
    """Test bytes with high value (255)"""
    b: bytes = b"\xff"
    return b[0]  # Expected: 255

def test_bytes_newline() -> int:
    """Test bytes with newline"""
    b: bytes = b"\n"
    return b[0]  # Expected: 10

def test_bytes_tab() -> int:
    """Test bytes with tab"""
    b: bytes = b"\t"
    return b[0]  # Expected: 9

def test_bytes_carriage_return() -> int:
    """Test bytes with carriage return"""
    b: bytes = b"\r"
    return b[0]  # Expected: 13

def test_bytes_backslash() -> int:
    """Test bytes with backslash"""
    b: bytes = b"\\"
    return b[0]  # Expected: 92

# ============ Hex escape sequences ============

def test_bytes_hex_00() -> int:
    """Test bytes with \\x00 escape"""
    b: bytes = b"\x00"
    return b[0]  # Expected: 0

def test_bytes_hex_7f() -> int:
    """Test bytes with \\x7f escape (DEL)"""
    b: bytes = b"\x7f"
    return b[0]  # Expected: 127

def test_bytes_hex_80() -> int:
    """Test bytes with \\x80 escape"""
    b: bytes = b"\x80"
    return b[0]  # Expected: 128

def test_bytes_hex_ff() -> int:
    """Test bytes with \\xff escape"""
    b: bytes = b"\xff"
    return b[0]  # Expected: 255

def test_bytes_hex_mixed() -> int:
    """Test bytes with mixed hex escapes"""
    b: bytes = b"\x01\x02\x03"
    return b[0] + b[1] + b[2]  # Expected: 6

# ============ Print tests (uses __str__/__repr__) ============

def test_bytes_print_simple() -> int:
    """Test printing simple bytes"""
    b: bytes = b"hello"
    print(b)  # Expected output: b'hello'
    return 1

def test_bytes_print_empty() -> int:
    """Test printing empty bytes"""
    b: bytes = b""
    print(b)  # Expected output: b''
    return 1

# ============ Edge cases ============

def test_bytes_sum_values() -> int:
    """Test summing byte values"""
    b: bytes = b"abc"
    return b[0] + b[1] + b[2]  # Expected: 97+98+99 = 294

def test_bytes_all_printable() -> int:
    """Test bytes with all printable ASCII"""
    b: bytes = b"ABCabc123"
    return len(b)  # Expected: 9

def test_bytes_boundary_values() -> int:
    """Test boundary values (31, 32, 126, 127)"""
    b: bytes = b"\x1f \x7e\x7f"
    return b[0] + b[1] + b[2] + b[3]  # Expected: 31+32+126+127 = 316

def main() -> int:
    failed: int = 0

    # __len__ tests
    if test_bytes_len_simple() != 5:
        print(1)
        failed = failed + 1
    if test_bytes_len_empty() != 0:
        print(2)
        failed = failed + 1
    if test_bytes_len_single() != 1:
        print(3)
        failed = failed + 1

    # __getitem__ tests
    if test_bytes_getitem_first() != 104:
        print(4)
        failed = failed + 1
    if test_bytes_getitem_last() != 111:
        print(5)
        failed = failed + 1
    if test_bytes_getitem_middle() != 108:
        print(6)
        failed = failed + 1
    if test_bytes_getitem_digit() != 48:
        print(7)
        failed = failed + 1
    if test_bytes_getitem_space() != 32:
        print(8)
        failed = failed + 1

    # Non-printable bytes
    if test_bytes_null_byte() != 0:
        print(9)
        failed = failed + 1
    if test_bytes_high_value() != 255:
        print(10)
        failed = failed + 1
    if test_bytes_newline() != 10:
        print(11)
        failed = failed + 1
    if test_bytes_tab() != 9:
        print(12)
        failed = failed + 1
    if test_bytes_carriage_return() != 13:
        print(13)
        failed = failed + 1
    if test_bytes_backslash() != 92:
        print(14)
        failed = failed + 1

    # Hex escape sequences
    if test_bytes_hex_00() != 0:
        print(15)
        failed = failed + 1
    if test_bytes_hex_7f() != 127:
        print(16)
        failed = failed + 1
    if test_bytes_hex_80() != 128:
        print(17)
        failed = failed + 1
    if test_bytes_hex_ff() != 255:
        print(18)
        failed = failed + 1
    if test_bytes_hex_mixed() != 6:
        print(19)
        failed = failed + 1

    # Print tests
    if test_bytes_print_simple() != 1:
        print(20)
        failed = failed + 1
    if test_bytes_print_empty() != 1:
        print(21)
        failed = failed + 1

    # Edge cases
    if test_bytes_sum_values() != 294:
        print(22)
        failed = failed + 1
    if test_bytes_all_printable() != 9:
        print(23)
        failed = failed + 1
    if test_bytes_boundary_values() != 316:
        print(24)
        failed = failed + 1

    if failed == 0:
        print(0)
    return failed

main()
