# Cannot call pow() with Float, Bytes, Bytes
x = pow(1.0, b"hello", b"hello")
