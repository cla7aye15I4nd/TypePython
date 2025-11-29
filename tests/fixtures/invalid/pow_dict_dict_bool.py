# Cannot call pow() with Dict[str, int], Dict[str, int], Bool
x = pow({"a": 1}, {"a": 1}, True)
