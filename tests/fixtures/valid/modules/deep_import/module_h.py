# Module H - imports I and J
import module_i
import module_j

def test() -> int:
    return module_i.test() + module_j.test() + 8
