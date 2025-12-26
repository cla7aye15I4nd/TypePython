# Test bytes type functionality

def test_bytes_literal() -> int:
    b: bytes = b"hello"
    return b[0]  # Should return 104 (ASCII 'h')

def test_bytes_index() -> int:
    b: bytes = b"abc"
    return b[1]  # Should return 98 (ASCII 'b')

def test_bytes_len() -> int:
    b: bytes = b"hello"
    return len(b)  # Should return 5

def test_bytes_empty_len() -> int:
    b: bytes = b""
    return len(b)  # Should return 0
