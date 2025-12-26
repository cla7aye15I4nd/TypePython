# Call module.function with wrong argument count
import algorithm.factorial as fact_module

def main() -> None:
    x: int = fact_module.factorial(1, 2)  # factorial takes 1 arg
