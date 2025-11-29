# Cannot call pow() with Bytes, Bool, Bytes
x = pow(b"hello", True, b"hello")
