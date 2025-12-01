# For loop with else clause
# Test 1: loop completes without break
for i in range(3):
    print(i)
else:
    print("Loop completed")

# Test 2: loop exits with break - else should not run
for i in range(5):
    if i == 2:
        break
    print(i)
else:
    print("Should not print")

print("Done")
