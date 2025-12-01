# Global statement
counter: int = 0

def increment() -> None:
    global counter
    counter = counter + 1

def get_counter() -> int:
    global counter
    return counter

increment()
increment()
increment()
print(get_counter())
# Expected: 3
