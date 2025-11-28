# Bytes in conditional logic
def describe_number(n: int) -> None:
    if n < 0:
        print(b"negative")
    else:
        if n == 0:
            print(b"zero")
        else:
            if n < 10:
                print(b"small positive")
            else:
                if n < 100:
                    print(b"medium positive")
                else:
                    print(b"large positive")

def categorize(value: int, threshold: int) -> None:
    category: bytes = b"unknown"

    if value < threshold:
        category = b"below"
    else:
        if value == threshold:
            category = b"equal"
        else:
            category = b"above"

    print(b"Category:", category)

def status_message(code: int) -> None:
    if code == 0:
        print(b"Success")
    else:
        if code == 1:
            print(b"Warning")
        else:
            if code == 2:
                print(b"Error")
            else:
                print(b"Unknown")

describe_number(-5)
describe_number(0)
describe_number(5)
describe_number(50)
describe_number(150)

categorize(30, 50)
categorize(50, 50)
categorize(70, 50)

status_message(0)
status_message(1)
status_message(2)
status_message(99)
