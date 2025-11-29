# Cannot call pow() with Str, Bytes, Str
x = pow("hello", b"hello", "hello")
