# Cannot power Dict[str, int] and Dict[str, int]
x: dict[str, int] = {"a": 1} ** {"a": 1}
