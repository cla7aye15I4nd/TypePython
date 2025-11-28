# Strings with loop constructs
def print_sequence(n: int) -> None:
    i: int = 1
    while i <= n:
        print(b"Iteration:", i)
        i = i + 1

def countdown(start: int) -> None:
    current: int = start
    while current > 0:
        print(b"Countdown:", current)
        current = current - 1
    print(b"Blastoff!")

def print_multiples(n: int, limit: int) -> None:
    i: int = 1
    while i <= limit:
        result: int = n * i
        print(b"Multiple:", result)
        i = i + 1

def labeled_fibonacci(n: int) -> None:
    a: int = 0
    b: int = 1
    count: int = 0

    while count < n:
        print(b"Fib:", a)
        temp: int = a + b
        a = b
        b = temp
        count = count + 1

print_sequence(5)
countdown(3)
print_multiples(7, 5)
labeled_fibonacci(8)
