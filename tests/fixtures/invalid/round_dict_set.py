# Cannot call round() with Dict[str, int], Set[int]
x = round({"a": 1}, {1, 2, 3})
