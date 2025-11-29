# Cannot call pow() with Str, Bytes, Bytes
x = pow("hello", b"hello", b"hello")
