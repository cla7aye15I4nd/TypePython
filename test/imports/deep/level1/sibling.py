# Module that imports from sibling packages

# Import from sibling's child (level2)
from .level2.utils import get_l2_value

# Import from parent (deep)
from ..utils import get_deep_value

def get_sibling_sum() -> int:
    return get_l2_value() + get_deep_value()  # 200 + 1000 = 1200
