# Cannot call pow() with Dict[str, int], Dict[str, int], Int
x = pow({"a": 1}, {"a": 1}, 1)
