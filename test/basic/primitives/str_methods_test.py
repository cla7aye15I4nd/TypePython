# Comprehensive test suite for Phase 3: String Methods
# Tests: Case conversion, whitespace ops, search, replace, character classification

# ============ Case Conversion Tests ============

def test_lower_ascii() -> str:
    """Test lower() on ASCII string"""
    s: str = "HELLO WORLD"
    return s.lower()  # Expected: hello world

def test_upper_ascii() -> str:
    """Test upper() on ASCII string"""
    s: str = "hello world"
    return s.upper()  # Expected: HELLO WORLD

def test_lower_unicode() -> str:
    """Test lower() on Unicode (Cyrillic)"""
    s: str = "ПРИВЕТ"
    return s.lower()  # Expected: привет

def test_upper_unicode() -> str:
    """Test upper() on Unicode (Cyrillic)"""
    s: str = "привет"
    return s.upper()  # Expected: ПРИВЕТ

def test_lower_mixed() -> str:
    """Test lower() on mixed ASCII and Unicode"""
    s: str = "Hello МИРHELLO"
    return s.lower()  # Expected: hello мирhello

# ============ Whitespace Operations ============

def test_strip_both() -> str:
    """Test strip() removes leading and trailing whitespace"""
    s: str = "  hello world  "
    return s.strip()  # Expected: hello world

def test_strip_leading_only() -> str:
    """Test strip() removes leading whitespace"""
    s: str = "  hello"
    return s.strip()  # Expected: hello

def test_strip_trailing_only() -> str:
    """Test strip() removes trailing whitespace"""
    s: str = "hello  "
    return s.strip()  # Expected: hello

def test_strip_tabs_newlines() -> str:
    """Test strip() removes tabs and newlines"""
    s: str = "\t\nhello\n\t"
    return s.strip()  # Expected: hello

def test_strip_none() -> str:
    """Test strip() on string with no whitespace"""
    s: str = "hello"
    return s.strip()  # Expected: hello

# ============ String Search Tests ============

def test_find_present() -> int:
    """Test find() when substring is present"""
    s: str = "hello world"
    return s.find("world")  # Expected: 6

def test_find_absent() -> int:
    """Test find() when substring is absent"""
    s: str = "hello world"
    return s.find("xyz")  # Expected: -1

def test_find_at_start() -> int:
    """Test find() when substring is at start"""
    s: str = "hello world"
    return s.find("hello")  # Expected: 0

def test_find_empty() -> int:
    """Test find() with empty substring"""
    s: str = "hello"
    return s.find("")  # Expected: 0

def test_startswith_true() -> bool:
    """Test startswith() returns true"""
    s: str = "hello world"
    return s.startswith("hello")  # Expected: True

def test_startswith_false() -> bool:
    """Test startswith() returns false"""
    s: str = "hello world"
    return s.startswith("world")  # Expected: False

def test_endswith_true() -> bool:
    """Test endswith() returns true"""
    s: str = "hello world"
    return s.endswith("world")  # Expected: True

def test_endswith_false() -> bool:
    """Test endswith() returns false"""
    s: str = "hello world"
    return s.endswith("hello")  # Expected: False

# ============ String Replacement Tests ============

def test_replace_single() -> str:
    """Test replace() single occurrence"""
    s: str = "hello world"
    return s.replace("world", "Python")  # Expected: hello Python

def test_replace_multiple() -> str:
    """Test replace() multiple occurrences"""
    s: str = "a,b,c,d"
    return s.replace(",", ";")  # Expected: a;b;c;d

def test_replace_not_found() -> str:
    """Test replace() when old substring not found"""
    s: str = "hello"
    return s.replace("xyz", "abc")  # Expected: hello

def test_replace_with_longer() -> str:
    """Test replace() with longer replacement"""
    s: str = "hi"
    return s.replace("hi", "hello")  # Expected: hello

def test_replace_with_shorter() -> str:
    """Test replace() with shorter replacement"""
    s: str = "hello"
    return s.replace("hello", "hi")  # Expected: hi

def test_replace_unicode() -> str:
    """Test replace() with Unicode"""
    s: str = "你好世界"
    return s.replace("世界", "朋友")  # Expected: 你好朋友

# ============ Character Classification Tests ============

def test_isalpha_true_ascii() -> bool:
    """Test isalpha() on ASCII letters"""
    return "abc".isalpha()  # Expected: True

def test_isalpha_false_digit() -> bool:
    """Test isalpha() on digits"""
    return "123".isalpha()  # Expected: False

def test_isalpha_false_mixed() -> bool:
    """Test isalpha() on mixed alphanumeric"""
    return "abc123".isalpha()  # Expected: False

def test_isalpha_true_unicode() -> bool:
    """Test isalpha() on Unicode letters (Chinese)"""
    return "你好".isalpha()  # Expected: True

def test_isdigit_true_ascii() -> bool:
    """Test isdigit() on ASCII digits"""
    return "123".isdigit()  # Expected: True

def test_isdigit_false_alpha() -> bool:
    """Test isdigit() on letters"""
    return "abc".isdigit()  # Expected: False

def test_isdigit_true_unicode() -> bool:
    """Test isdigit() on Unicode digits (Arabic-Indic)"""
    return "١٢٣".isdigit()  # Expected: True

def test_isspace_true() -> bool:
    """Test isspace() on spaces"""
    return "   ".isspace()  # Expected: True

def test_isspace_false() -> bool:
    """Test isspace() on non-space"""
    return "abc".isspace()  # Expected: False

def test_isspace_true_tabs() -> bool:
    """Test isspace() on tabs and newlines"""
    return "\t\n ".isspace()  # Expected: True

def main() -> int:
    failed: int = 0

    # Case conversion tests
    if test_lower_ascii() != "hello world":
        print(1)
        failed = failed + 1
    if test_upper_ascii() != "HELLO WORLD":
        print(2)
        failed = failed + 1
    if test_lower_unicode() != "привет":
        print(3)
        failed = failed + 1
    if test_upper_unicode() != "ПРИВЕТ":
        print(4)
        failed = failed + 1
    if test_lower_mixed() != "hello мирhello":
        print(5)
        failed = failed + 1

    # Whitespace operations
    if test_strip_both() != "hello world":
        print(6)
        failed = failed + 1
    if test_strip_leading_only() != "hello":
        print(7)
        failed = failed + 1
    if test_strip_trailing_only() != "hello":
        print(8)
        failed = failed + 1
    if test_strip_tabs_newlines() != "hello":
        print(9)
        failed = failed + 1
    if test_strip_none() != "hello":
        print(10)
        failed = failed + 1

    # String search tests
    if test_find_present() != 6:
        print(11)
        failed = failed + 1
    if test_find_absent() != -1:
        print(12)
        failed = failed + 1
    if test_find_at_start() != 0:
        print(13)
        failed = failed + 1
    if test_find_empty() != 0:
        print(14)
        failed = failed + 1
    if test_startswith_true() != True:
        print(15)
        failed = failed + 1
    if test_startswith_false() != False:
        print(16)
        failed = failed + 1
    if test_endswith_true() != True:
        print(17)
        failed = failed + 1
    if test_endswith_false() != False:
        print(18)
        failed = failed + 1

    # String replacement tests
    if test_replace_single() != "hello Python":
        print(19)
        failed = failed + 1
    if test_replace_multiple() != "a;b;c;d":
        print(20)
        failed = failed + 1
    if test_replace_not_found() != "hello":
        print(21)
        failed = failed + 1
    if test_replace_with_longer() != "hello":
        print(22)
        failed = failed + 1
    if test_replace_with_shorter() != "hi":
        print(23)
        failed = failed + 1
    if test_replace_unicode() != "你好朋友":
        print(24)
        failed = failed + 1

    # Character classification tests
    if test_isalpha_true_ascii() != True:
        print(25)
        failed = failed + 1
    if test_isalpha_false_digit() != False:
        print(26)
        failed = failed + 1
    if test_isalpha_false_mixed() != False:
        print(27)
        failed = failed + 1
    if test_isalpha_true_unicode() != True:
        print(28)
        failed = failed + 1
    if test_isdigit_true_ascii() != True:
        print(29)
        failed = failed + 1
    if test_isdigit_false_alpha() != False:
        print(30)
        failed = failed + 1
    if test_isdigit_true_unicode() != True:
        print(31)
        failed = failed + 1
    if test_isspace_true() != True:
        print(32)
        failed = failed + 1
    if test_isspace_false() != False:
        print(33)
        failed = failed + 1
    if test_isspace_true_tabs() != True:
        print(34)
        failed = failed + 1

    if failed == 0:
        print(0)
    else:
        print(failed)

    return failed
