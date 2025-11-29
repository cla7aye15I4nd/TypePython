# Cannot call pow() with Dict[str, int], Int, Dict[str, int]
x = pow({"a": 1}, 1, {"a": 1})
