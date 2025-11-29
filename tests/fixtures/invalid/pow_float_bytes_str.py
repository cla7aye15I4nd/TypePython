# Cannot call pow() with Float, Bytes, Str
x = pow(1.0, b"hello", "hello")
