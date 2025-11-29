# Cannot call pow() with Bytes, Str, Str
x = pow(b"hello", "hello", "hello")
