# Comprehensive tests for str class
# Tests: __len__, __str__, __repr__, __getitem__, special characters

# ============ __len__ tests ============

def test_str_len_simple() -> int:
    """Test len() on simple string"""
    s: str = "hello"
    return len(s)  # Expected: 5

def test_str_len_empty() -> int:
    """Test len() on empty string"""
    s: str = ""
    return len(s)  # Expected: 0

def test_str_len_single() -> int:
    """Test len() on single character"""
    s: str = "a"
    return len(s)  # Expected: 1

def test_str_len_spaces() -> int:
    """Test len() on string with spaces"""
    s: str = "   "
    return len(s)  # Expected: 3

def test_str_len_long() -> int:
    """Test len() on longer string"""
    s: str = "The quick brown fox"
    return len(s)  # Expected: 19

# ============ Print tests (uses __str__) ============

def test_str_print_simple() -> int:
    """Test printing simple string"""
    s: str = "hello world"
    print(s)  # Expected output: hello world
    return 1

def test_str_print_empty() -> int:
    """Test printing empty string"""
    s: str = ""
    print(s)  # Expected output: (empty line)
    return 1

def test_str_print_with_spaces() -> int:
    """Test printing string with leading/trailing spaces"""
    s: str = "  spaced  "
    print(s)  # Expected output:   spaced
    return 1

# ============ Special characters ============

def test_str_newline() -> int:
    """Test string with newline character"""
    s: str = "hello\nworld"
    return len(s)  # Expected: 11

def test_str_tab() -> int:
    """Test string with tab character"""
    s: str = "a\tb"
    return len(s)  # Expected: 3

def test_str_backslash() -> int:
    """Test string with backslash"""
    s: str = "path\\to\\file"
    return len(s)  # Expected: 12

def test_str_mixed_escapes() -> int:
    """Test string with mixed escape sequences"""
    s: str = "a\tb\nc\\"
    return len(s)  # Expected: 6

# ============ String variable assignment ============

def test_str_assignment() -> int:
    """Test string assignment to multiple variables"""
    s1: str = "hello"
    s2: str = s1
    print(s2)  # Expected output: hello
    return len(s1) + len(s2)  # Expected: 10

def test_str_concat_vars() -> int:
    """Test multiple string variables"""
    a: str = "abc"
    b: str = "def"
    return len(a) + len(b)  # Expected: 6

# ============ Edge cases ============

def test_str_digits() -> int:
    """Test string containing only digits"""
    s: str = "12345"
    return len(s)  # Expected: 5

def test_str_punctuation() -> int:
    """Test string with punctuation"""
    s: str = "!@#$%^&*()"
    return len(s)  # Expected: 10

def test_str_quote_in_string() -> int:
    """Test string with escaped quote"""
    s: str = "he said \"hello\""
    return len(s)  # Expected: 15

def main() -> int:
    failed: int = 0

    # __len__ tests
    if test_str_len_simple() != 5:
        print(1)
        failed = failed + 1
    if test_str_len_empty() != 0:
        print(2)
        failed = failed + 1
    if test_str_len_single() != 1:
        print(3)
        failed = failed + 1
    if test_str_len_spaces() != 3:
        print(4)
        failed = failed + 1
    if test_str_len_long() != 19:
        print(5)
        failed = failed + 1

    # Print tests
    if test_str_print_simple() != 1:
        print(6)
        failed = failed + 1
    if test_str_print_empty() != 1:
        print(7)
        failed = failed + 1
    if test_str_print_with_spaces() != 1:
        print(8)
        failed = failed + 1

    # Special characters
    if test_str_newline() != 11:
        print(9)
        failed = failed + 1
    if test_str_tab() != 3:
        print(10)
        failed = failed + 1
    if test_str_backslash() != 12:
        print(11)
        failed = failed + 1
    if test_str_mixed_escapes() != 6:
        print(12)
        failed = failed + 1

    # String variable assignment
    if test_str_assignment() != 10:
        print(13)
        failed = failed + 1
    if test_str_concat_vars() != 6:
        print(14)
        failed = failed + 1

    # Edge cases
    if test_str_digits() != 5:
        print(15)
        failed = failed + 1
    if test_str_punctuation() != 10:
        print(16)
        failed = failed + 1
    if test_str_quote_in_string() != 15:
        print(17)
        failed = failed + 1

    if failed == 0:
        print(0)
    return failed

main()
