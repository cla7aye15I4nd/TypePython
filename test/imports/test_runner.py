# Import tests module
from imports.helper import *
from imports.math_utils import square, double as double_value
from imports.deep.level1.level2.level3.core import test_relative_imports
from .deep.level1.sibling import get_sibling_sum

# Test "from . import module" syntax
from . import helper, math_utils as math_utils_alias

# Test global variable imports
from imports.math_utils import PI_APPROX, MAX_VALUE as MAX_VAL, counter
from .helper import helper_value

def test() -> int:
    # Test imported functions
    print(add(3, 4))       # 7
    print(multiply(3, 4))  # 12
    print(square(5))       # 25
    print(double_value(5))       # 10

    # Test "from . import module" syntax - access via module namespace
    print(helper.add(10, 20))        # 30
    print(helper.multiply(6, 7))     # 42
    print(math_utils_alias.square(6))      # 36
    print(math_utils_alias.double(15))     # 30

    # Complex relative import tests
    print(test_relative_imports())  # 1630
    print(get_sibling_sum())        # 1200

    # Test global variable imports
    print(HELPER_CONSTANT)           # 42 (from star import)
    print(helper_value)              # 100 (direct import from relative)
    print(PI_APPROX)                 # 3 (direct import)
    print(MAX_VAL)                   # 999 (aliased import)
    print(counter)                   # 50 (direct import)

    # Test global variables accessed via module namespace
    print(helper.HELPER_CONSTANT)    # 42
    print(helper.helper_value)       # 100
    print(math_utils_alias.PI_APPROX)      # 3
    print(math_utils_alias.MAX_VALUE)      # 999
    print(math_utils_alias.counter)        # 50

    # Test functions that use module-level globals internally
    print(helper.get_helper_constant())    # 42
    print(math_utils_alias.get_max())      # 999

    return 0
