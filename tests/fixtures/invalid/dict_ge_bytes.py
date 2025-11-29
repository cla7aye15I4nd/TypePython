# Cannot compare with >= Dict[str, int] and Bytes
x: bool = {"a": 1} >= b"hello"
