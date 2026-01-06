# Comprehensive Unicode tests for str class
# Tests: Unicode characters, emoji, concatenation, multi-script support

# ============ Unicode character display tests ============

def test_unicode_chinese() -> int:
    """Test Chinese characters"""
    s: str = "你好世界"
    print(s)  # Expected output: 你好世界
    return 1

def test_unicode_emoji() -> int:
    """Test emoji characters"""
    s: str = "👋🌍🐍⚡"
    print(s)  # Expected output: 👋🌍🐍⚡
    return 1

def test_unicode_arabic() -> int:
    """Test Arabic (RTL) text"""
    s: str = "مرحبا"
    print(s)  # Expected output: مرحبا
    return 1

def test_unicode_cyrillic() -> int:
    """Test Cyrillic text"""
    s: str = "Привет"
    print(s)  # Expected output: Привет
    return 1

def test_unicode_devanagari() -> int:
    """Test Devanagari script"""
    s: str = "नमस्ते"
    print(s)  # Expected output: नमस्ते
    return 1

# ============ Unicode len() tests ============

def test_unicode_len_chinese() -> int:
    """Test len() on Chinese characters"""
    s: str = "你好世界"
    return len(s)  # Expected: 4

def test_unicode_len_emoji() -> int:
    """Test len() on emoji"""
    s: str = "👋🌍🐍⚡"
    return len(s)  # Expected: 4

def test_unicode_len_complex_emoji() -> int:
    """Test len() on complex emoji (family with ZWJ)"""
    s: str = "👨‍👩‍👧‍👦"
    return len(s)  # Expected: 7 (4 person emoji + 3 ZWJ codepoints)

def test_unicode_len_mixed() -> int:
    """Test len() on mixed ASCII and Unicode"""
    s: str = "Hello世界"
    return len(s)  # Expected: 7 (5 ASCII + 2 Chinese)

# ============ Unicode concatenation tests ============

def test_unicode_concat_chinese() -> int:
    """Test concatenation with Chinese"""
    a: str = "Hello "
    b: str = "世界"
    c: str = a + b
    print(c)  # Expected output: Hello 世界
    return len(c)  # Expected: 8

def test_unicode_concat_emoji() -> int:
    """Test concatenation with emoji"""
    a: str = "Python"
    b: str = "🐍"
    c: str = a + b
    print(c)  # Expected output: Python🐍
    return len(c)  # Expected: 7

def test_unicode_concat_multi() -> int:
    """Test multiple concatenations"""
    a: str = "Hello "
    b: str = "مرحبا "
    c: str = "Привет"
    d: str = a + b + c
    print(d)  # Expected output: Hello مرحبا Привет
    return 1

def test_unicode_concat_emoji_chain() -> int:
    """Test chained emoji concatenation"""
    s: str = "🐍" + "⚡" + "🚀"
    print(s)  # Expected output: 🐍⚡🚀
    return len(s)  # Expected: 3

# ============ Multi-script tests ============

def test_unicode_multiscript() -> int:
    """Test multiple scripts in one string"""
    s: str = "Hello مرحبا Привет 你好 नमस्ते"
    print(s)  # Expected output: Hello مرحبا Привет 你好 नमस्ते
    return len(s)  # Expected: 25

def test_unicode_mixed_content() -> int:
    """Test mixed ASCII, Unicode, and emoji"""
    s: str = "Python🐍 + TypePython⚡"
    print(s)  # Expected output: Python🐍 + TypePython⚡
    return len(s)  # Expected: 21

# ============ Edge cases ============

def test_unicode_empty_concat() -> int:
    """Test concatenating empty string with Unicode"""
    a: str = ""
    b: str = "你好"
    c: str = a + b
    return len(c)  # Expected: 2

def test_unicode_ascii_optimized() -> int:
    """Test that ASCII-only strings are still fast"""
    s: str = "Hello World"
    return len(s)  # Expected: 11 (should use fast path)

def test_unicode_special_chars() -> int:
    """Test Unicode with special characters"""
    s: str = "Tab:\t中文\nNewline"
    return len(s)  # Expected: 15

# ============ Emoji variant tests ============

def test_unicode_emoji_skin_tone() -> int:
    """Test emoji with skin tone modifier"""
    s: str = "👋🏽"
    print(s)  # Expected output: 👋🏽
    return len(s)  # Expected: 2 (wave + skin tone modifier codepoints)

def test_unicode_emoji_flag() -> int:
    """Test flag emoji (regional indicators)"""
    s: str = "🇺🇸"
    print(s)  # Expected output: 🇺🇸
    return len(s)  # Expected: 2 (two regional indicator codepoints)

# ============ Real-world scenarios ============

def test_unicode_sentence() -> int:
    """Test realistic multilingual sentence"""
    s: str = "Welcome! 欢迎! Willkommen! ברוכים הבאים!"
    print(s)
    return 1

def test_unicode_programming() -> int:
    """Test programming-related Unicode"""
    s: str = "λ函数 → 🚀"
    print(s)
    return len(s)  # Expected: 7

def main() -> int:
    failed: int = 0

    # Display tests
    if test_unicode_chinese() != 1:
        print(1)
        failed = failed + 1
    if test_unicode_emoji() != 1:
        print(2)
        failed = failed + 1
    if test_unicode_arabic() != 1:
        print(3)
        failed = failed + 1
    if test_unicode_cyrillic() != 1:
        print(4)
        failed = failed + 1
    if test_unicode_devanagari() != 1:
        print(5)
        failed = failed + 1

    # len() tests
    if test_unicode_len_chinese() != 4:
        print(6)
        failed = failed + 1
    if test_unicode_len_emoji() != 4:
        print(7)
        failed = failed + 1
    if test_unicode_len_complex_emoji() != 7:
        print(8)
        failed = failed + 1
    if test_unicode_len_mixed() != 7:
        print(9)
        failed = failed + 1

    # Concatenation tests
    if test_unicode_concat_chinese() != 8:
        print(10)
        failed = failed + 1
    if test_unicode_concat_emoji() != 7:
        print(11)
        failed = failed + 1
    if test_unicode_concat_multi() != 1:
        print(12)
        failed = failed + 1
    if test_unicode_concat_emoji_chain() != 3:
        print(13)
        failed = failed + 1

    # Multi-script tests
    if test_unicode_multiscript() != 25:
        print(14)
        failed = failed + 1
    if test_unicode_mixed_content() != 21:
        print(15)
        failed = failed + 1

    # Edge cases
    if test_unicode_empty_concat() != 2:
        print(16)
        failed = failed + 1
    if test_unicode_ascii_optimized() != 11:
        print(17)
        failed = failed + 1
    if test_unicode_special_chars() != 15:
        print(18)
        failed = failed + 1

    # Emoji variants
    if test_unicode_emoji_skin_tone() != 2:
        print(19)
        failed = failed + 1
    if test_unicode_emoji_flag() != 2:
        print(20)
        failed = failed + 1

    # Real-world scenarios
    if test_unicode_sentence() != 1:
        print(21)
        failed = failed + 1
    if test_unicode_programming() != 7:
        print(22)
        failed = failed + 1

    if failed == 0:
        print(0)
    else:
        print(failed)

    return failed
