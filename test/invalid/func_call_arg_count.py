# Function call with wrong argument count
def add(x: int, y: int) -> int:
    return x + y

def main() -> None:
    result: int = add(1)
    print(result)
