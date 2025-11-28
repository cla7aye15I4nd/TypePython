# Module K - imports L and M, creates cycle with I and J
import module_l
import module_m
import module_i

def test() -> int:
    return module_l.test() + module_m.test() + 11
