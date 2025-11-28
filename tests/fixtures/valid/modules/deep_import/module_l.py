# Module L - imports M and N
import module_m
import module_n

def test() -> int:
    return module_m.test() + module_n.test() + 12
