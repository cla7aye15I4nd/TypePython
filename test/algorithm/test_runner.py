# Algorithm tests module
from algorithm.factorial import factorial
from algorithm.fibonacci import fibonacci

def test() -> int:
    # Algorithm tests
    print(factorial(5))      # 120
    print(fibonacci(10))     # 55

    return 0
