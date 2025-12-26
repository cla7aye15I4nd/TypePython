# Global variables for testing imports
HELPER_CONSTANT: int = 42
helper_value: int = 100

def add(a: int, b: int) -> int:
    return a + b

def multiply(a: int, b: int) -> int:
    return a * b

def get_helper_constant() -> int:
    return HELPER_CONSTANT
