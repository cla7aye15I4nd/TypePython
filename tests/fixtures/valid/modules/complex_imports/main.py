import calculator
import utils

result1: int = calculator.compute(5)
result2: int = calculator.complex_compute(3, 7)
result3: int = utils.helper1(10) + utils.helper2(10) + utils.helper3(10)

print(b"Calculator compute:", result1)
print(b"Complex compute:", result2)
print(b"Direct utils:", result3)
