# Cannot call pow() with Dict[str, int], Dict[str, int], Float
x = pow({"a": 1}, {"a": 1}, 1.0)
