# For loop over bytes
b: bytes = b"hello"
total: int = 0
for x in b:
    total = total + x
print(total)
# h=104, e=101, l=108, l=108, o=111 = 532
