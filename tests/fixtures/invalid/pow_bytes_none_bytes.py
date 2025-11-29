# Cannot call pow() with Bytes, None, Bytes
x = pow(b"hello", None, b"hello")
