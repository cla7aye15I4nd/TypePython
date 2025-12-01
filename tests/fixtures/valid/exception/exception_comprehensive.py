# Comprehensive exception handling tests

# Test multiple exception handlers with various types
result: int = 0

# Test 1: Try with else clause
try:
    x: int = 10
    y: int = 5
except ValueError:
    result = 1
else:
    result = 2
print(result)

# Test 2: KeyError
result = 0
try:
    raise KeyError
except KeyError:
    result = 3
print(result)

# Test 3: IndexError
result = 0
try:
    raise IndexError
except IndexError:
    result = 4
print(result)

# Test 4: RuntimeError
result = 0
try:
    raise RuntimeError
except RuntimeError:
    result = 5
print(result)

# Test 5: Finally only (no except)
result = 0
try:
    result = 6
finally:
    result = result + 1
print(result)

# Test 6: Multiple handlers with finally
result = 0
try:
    raise TypeError
except ValueError:
    result = 10
except TypeError:
    result = 20
except RuntimeError:
    result = 30
finally:
    result = result + 1
print(result)
