# Module N - imports O and P
import module_o
import module_p

def test() -> int:
    return module_o.test() + module_p.test() + 14
