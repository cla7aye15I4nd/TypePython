# Try/except/else/finally all together
x: int = 10
y: int = 2

try:
    result: int = x // y
except:
    print("Error")
else:
    print("Result:", result)
finally:
    print("Cleanup done")
