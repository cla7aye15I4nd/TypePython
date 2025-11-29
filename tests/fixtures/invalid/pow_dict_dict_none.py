# Cannot call pow() with Dict[str, int], Dict[str, int], None
x = pow({"a": 1}, {"a": 1}, None)
