# Module J - imports K and L
import module_k
import module_l

def test() -> int:
    return module_k.test() + module_l.test() + 10
