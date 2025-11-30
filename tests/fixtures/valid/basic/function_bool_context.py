# Test function in boolean context
def foo() -> None:
    pass

# Function in if statement (always truthy)
if foo:
    print("function is truthy")
else:
    print("function is falsy")

# Function in while (break after first iteration)
count: int = 0
while foo:
    print("function truthy in while")
    count = count + 1
    if count > 0:
        break

print("function bool tests passed!")
