# Module D - imports E and F
import module_e
import module_f

def test() -> int:
    return module_e.test() + module_f.test() + 4
