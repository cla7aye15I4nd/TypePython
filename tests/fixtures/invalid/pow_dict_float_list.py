# Cannot call pow() with Dict[str, int], Float, List[int]
x = pow({"a": 1}, 1.0, [1, 2, 3])
