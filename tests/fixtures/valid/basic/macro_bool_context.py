# Test macro (print, min, max, abs) in boolean context
if print:
    print("print macro is truthy")

# min/max are also macros
if min:
    print("min is truthy")

if max:
    print("max is truthy")

if abs:
    print("abs is truthy")

print("macro bool tests passed!")
