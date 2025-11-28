import utils

def compute(x: int) -> int:
    a: int = utils.helper1(x)
    b: int = utils.helper2(x)
    c: int = utils.helper3(x)
    return a + b + c

def complex_compute(x: int, y: int) -> int:
    result1: int = utils.helper1(x) + utils.helper2(y)
    result2: int = utils.helper3(x + y)
    return result1 + result2
