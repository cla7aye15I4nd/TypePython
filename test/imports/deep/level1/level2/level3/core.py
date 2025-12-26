# Module at deepest level - tests various relative imports

# Import from same package (level3)
from .utils import get_l3_value

# Import from parent package (level2)
from ..utils import get_l2_value

# Import from grandparent package (level1)
from ...utils import get_l1_value

# Import from great-grandparent package (deep)
from ....utils import get_deep_value

# Import from root imports package (5 levels up)
from .....helper import add

def test_relative_imports() -> int:
    # Test all the relative imports work
    result: int = get_l3_value() + get_l2_value() + get_l1_value() + get_deep_value() + add(10, 20)
    return result  # 300 + 200 + 100 + 1000 + 30 = 1630
