# Cannot call pow() with Int, Bytes, Bytes
x = pow(1, b"hello", b"hello")
