# Cannot call pow() with Str, Float, Bytes
x = pow("hello", 1.0, b"hello")
