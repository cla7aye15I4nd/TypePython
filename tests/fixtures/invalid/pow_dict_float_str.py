# Cannot call pow() with Dict[str, int], Float, Str
x = pow({"a": 1}, 1.0, "hello")
