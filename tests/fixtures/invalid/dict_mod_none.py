# Cannot modulo Dict[str, int] and None
x: dict[str, int] = {"a": 1} % None
