# Cannot call divmod() with Dict[str, int], Set[int]
x = divmod({"a": 1}, {1, 2, 3})
