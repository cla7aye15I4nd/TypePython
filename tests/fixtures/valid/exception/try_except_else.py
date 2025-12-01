# Try/except/else pattern
x: int = 10
y: int = 2

try:
    result: int = x // y
except:
    print("Error occurred")
else:
    print("Success:", result)
