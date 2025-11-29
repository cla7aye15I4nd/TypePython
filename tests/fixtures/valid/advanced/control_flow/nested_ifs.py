# Test deeply nested conditionals
x: int = 5

if x > 0:
    if x > 3:
        if x > 4:
            print(b"deep")
        else:
            print(b"level3")
    else:
        print(b"level2")
else:
    print(b"negative")
