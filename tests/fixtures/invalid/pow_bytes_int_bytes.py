# Cannot call pow() with Bytes, Int, Bytes
x = pow(b"hello", 1, b"hello")
