# Return type mismatch - returning str when int expected
def get_number() -> int:
    return "hello"

def main() -> None:
    x: int = get_number()
    print(x)
