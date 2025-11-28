# Module G - imports H and I, creates cycle with E and F
import module_h
import module_i
import module_e

def test() -> int:
    return module_h.test() + module_i.test() + 7
