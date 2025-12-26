# Function call with wrong argument type
def add(x: int, y: int) -> int:
    return x + y

def main() -> None:
    result: int = add("hello", 2)
    print(result)
