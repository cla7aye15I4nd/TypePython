# Test continue statement in while loop
counter: int = 0
while counter < 5:
    counter = counter + 1
    if counter == 3:
        continue
    print("Counter:", counter)
print("Done!")
