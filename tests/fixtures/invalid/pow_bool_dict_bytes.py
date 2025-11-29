# Cannot call pow() with Bool, Dict[str, int], Bytes
x = pow(True, {"a": 1}, b"hello")
