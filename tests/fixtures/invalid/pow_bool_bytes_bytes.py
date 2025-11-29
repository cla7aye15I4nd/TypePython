# Cannot call pow() with Bool, Bytes, Bytes
x = pow(True, b"hello", b"hello")
