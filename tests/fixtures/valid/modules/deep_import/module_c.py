# Module C - imports D and E, creates cycle with A and B
import module_d
import module_e
import module_a
import module_b

def test() -> int:
    return module_d.test() + module_e.test() + 3
