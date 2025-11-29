# Cannot call pow() with Bytes, Bytes, Str
x = pow(b"hello", b"hello", "hello")
