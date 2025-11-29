# Cannot call pow() with Float, Dict[str, int], Bytes
x = pow(1.0, {"a": 1}, b"hello")
