# Cannot call pow() with Bytes, Float, Str
x = pow(b"hello", 1.0, "hello")
