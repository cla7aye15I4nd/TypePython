# Test break statement in while loop
counter: int = 0
while counter < 10:
    print("Counter:", counter)
    if counter == 3:
        break
    counter = counter + 1
print("Loop exited at:", counter)
