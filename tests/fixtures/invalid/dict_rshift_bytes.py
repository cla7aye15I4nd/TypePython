# Cannot right shift Dict[str, int] and Bytes
x: dict[str, int] = {"a": 1} >> b"hello"
