# Strings in conditional logic
def describe_number(n: int) -> None:
    if n < 0:
        print("negative")
    else:
        if n == 0:
            print("zero")
        else:
            if n < 10:
                print("small positive")
            else:
                if n < 100:
                    print("medium positive")
                else:
                    print("large positive")

def categorize(value: int, threshold: int) -> None:
    category: str = "unknown"

    if value < threshold:
        category = "below"
    else:
        if value == threshold:
            category = "equal"
        else:
            category = "above"

    print("Category:", category)

def status_message(code: int) -> None:
    if code == 0:
        print("Success")
    else:
        if code == 1:
            print("Warning")
        else:
            if code == 2:
                print("Error")
            else:
                print("Unknown")

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
