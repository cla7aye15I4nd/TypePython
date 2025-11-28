# Module B - imports C and D, creates cycle with A
import module_c
import module_d
import module_a

def test() -> int:
    return module_c.test() + module_d.test() + 2
