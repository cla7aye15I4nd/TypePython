# Cannot call pow() with Dict[str, int], Float, Dict[str, int]
x = pow({"a": 1}, 1.0, {"a": 1})
