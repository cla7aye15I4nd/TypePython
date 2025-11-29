# Cannot call pow() with Bytes, Bytes, Float
x = pow(b"hello", b"hello", 1.0)
