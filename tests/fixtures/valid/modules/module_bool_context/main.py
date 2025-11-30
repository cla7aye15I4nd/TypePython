# Test module in boolean context
import helper

# Module in if statement (always truthy)
if helper:
    print("module is truthy")
else:
    print("module is falsy")

# Use the module to show it works
helper.greet()

print("module bool tests passed!")
