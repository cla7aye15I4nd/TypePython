# Class without __init__ called with arguments
class Empty:
    value: int

def main() -> None:
    e: Empty = Empty(1)
    print(e.value)
