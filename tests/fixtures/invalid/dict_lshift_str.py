# Cannot left shift Dict[str, int] and Str
x: dict[str, int] = {"a": 1} << "hello"
