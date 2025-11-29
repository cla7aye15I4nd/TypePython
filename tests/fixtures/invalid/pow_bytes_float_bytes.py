# Cannot call pow() with Bytes, Float, Bytes
x = pow(b"hello", 1.0, b"hello")
