# Functions with None return type and side effects
counter: int = 0

def increment_counter() -> None:
    global counter
    counter = counter + 1

def add_to_counter(value: int) -> None:
    global counter
    counter = counter + value

def reset_counter() -> None:
    global counter
    counter = 0

def multiply_counter(factor: int) -> None:
    global counter
    counter = counter * factor

# Test void function calls
increment_counter()
increment_counter()
increment_counter()
print("Counter after 3 increments:", counter)

add_to_counter(10)
print("Counter after adding 10:", counter)

multiply_counter(2)
print("Counter after multiply by 2:", counter)

reset_counter()
print("Counter after reset:", counter)

add_to_counter(100)
print("Final counter:", counter)
