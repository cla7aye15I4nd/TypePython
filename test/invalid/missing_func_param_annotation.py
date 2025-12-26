# Function parameter without type annotation
def add(x, y: int) -> int:
    return x + y

def main() -> None:
    print(add(1, 2))
