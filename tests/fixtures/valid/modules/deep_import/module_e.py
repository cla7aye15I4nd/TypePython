# Module E - imports F and G, creates cycle with D
import module_f
import module_g
import module_d

def test() -> int:
    return module_f.test() + module_g.test() + 5
