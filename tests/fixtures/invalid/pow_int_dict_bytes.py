# Cannot call pow() with Int, Dict[str, int], Bytes
x = pow(1, {"a": 1}, b"hello")
