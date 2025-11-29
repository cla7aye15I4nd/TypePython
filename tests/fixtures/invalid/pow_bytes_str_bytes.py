# Cannot call pow() with Bytes, Str, Bytes
x = pow(b"hello", "hello", b"hello")
