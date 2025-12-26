# Call module.function with wrong argument type
import algorithm.factorial as fact_module

def main() -> None:
    x: int = fact_module.factorial(True)  # factorial expects int, not bool
